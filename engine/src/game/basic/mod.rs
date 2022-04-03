pub mod input;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use derive_more::{Deref, DerefMut};
use ggrs::{InputStatus, P2PSession, SpectatorSession, SyncTestSession};

use crate::core::maths::Vector2;
use crate::core::transform::Transform2;
use crate::core::EngineGGRSConfig;
use crate::game::input::*;

pub trait GameApp {
    fn insert_game(&mut self) -> &mut Self;
}
impl GameApp for App {
    fn insert_game(&mut self) -> &mut Self {
        self.add_startup_system(player_startup_system)
    }
}

pub fn game_scheduler() -> SystemStage {
    SystemStage::parallel()
        .with_system(move_player_system)
        .with_system(exit_on_esc_system)
}

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct Velocity(Vector2);

fn player_startup_system(
    mut commands: Commands,
    p2p_session: Option<Res<P2PSession<EngineGGRSConfig>>>,
    synctest_session: Option<Res<SyncTestSession<EngineGGRSConfig>>>,
    spectator_session: Option<Res<SpectatorSession<EngineGGRSConfig>>>,
    mut rollback_id_provider: ResMut<RollbackIdProvider>,
) {
    let num_players = p2p_session
        .map(|s| s.num_players())
        .or_else(|| synctest_session.map(|s| s.num_players()))
        .or_else(|| spectator_session.map(|s| s.num_players()))
        .expect("No GGRS session found");

    for handle in 0..num_players {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.27, 0.0),
                    custom_size: Some(Vec2::new(55.0, 55.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player { handle })
            .insert(Velocity::default())
            .insert(Transform2::default())
            .insert(Rollback::new(rollback_id_provider.next_id()));
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn move_player_system(
    mut query: Query<(&Player, &mut Velocity, &mut Transform2), With<Rollback>>,
    inputs: Res<Vec<(GameInput, InputStatus)>>,
) {
    for (player, mut velocity, mut transform2) in query.iter_mut() {
        let input_mask = inputs[player.handle as usize].0.mask;

        *velocity = Velocity::default();
        if input_mask & INPUT_UP != 0 {
            velocity.y += 2.into();
        }
        if input_mask & INPUT_DOWN != 0 {
            velocity.y += (-2).into();
        }
        if input_mask & INPUT_LEFT != 0 {
            velocity.x += (-2).into();
        }
        if input_mask & INPUT_RIGHT != 0 {
            velocity.x += 2.into();
        }

        transform2.pos.x += velocity.x;
        transform2.pos.y += velocity.y;
    }
}
