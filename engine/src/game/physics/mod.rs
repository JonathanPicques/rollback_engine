pub mod input;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use derive_more::{Deref, DerefMut};
use ggrs::{InputStatus, P2PSession, SpectatorSession, SyncTestSession};

use crate::core::maths::Vector2;
use crate::core::physics::*;
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
            .spawn()
            .insert(Player { handle })
            .insert(Rollback::new(rollback_id_provider.next_id()))
            .insert(Transform2::from_pos(Vector2::new(handle * 40, 0)))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.27, 0.0),
                    custom_size: Some(Vec2::new(7.0, 15.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(KinematicBodyBundle {
                collider: PhysicsCollider {
                    size: Vector2::new(7, 14),
                    layer: 0,
                    layer_mask: 1,
                },
                ..Default::default()
            });
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn move_player_system(
    mut query: Query<(&Player, &mut KinematicBody), With<Rollback>>,
    inputs: Res<Vec<(GameInput, InputStatus)>>,
) {
    for (player, mut kinematic_body) in query.iter_mut() {
        let input_mask = inputs[player.handle as usize].0.mask;

        kinematic_body.velocity.x = 0.into();
        kinematic_body.velocity.y = 0.into();
        if input_mask & INPUT_UP != 0 {
            kinematic_body.velocity.y += 2.into();
        }
        if input_mask & INPUT_DOWN != 0 {
            kinematic_body.velocity.y += (-2).into();
        }
        if input_mask & INPUT_LEFT != 0 {
            kinematic_body.velocity.x += (-2).into();
        }
        if input_mask & INPUT_RIGHT != 0 {
            kinematic_body.velocity.x += 2.into();
        }
    }
}
