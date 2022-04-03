pub mod maths;
pub mod physics;
pub mod transform;

use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use ggrs::Config;
use std::net::SocketAddr;

use crate::core::physics::*;
use crate::core::transform::{sync_transform_system, Transform2};
use crate::game::game_scheduler;
use crate::game::input::{game_input_system, GameInput};

#[derive(Eq, Hash, Clone, Debug, PartialEq, StageLabel)]
enum RollbackStages {
    Game,
    Physics,
    TransformSynchronization,
}

pub struct EngineConfig {
    pub window_title: String,
    pub window_width: f32,
    pub window_height: f32,
    pub update_frequency: usize,
}
impl EngineConfig {
    pub fn default() -> EngineConfig {
        EngineConfig {
            window_title: "Rollback Engine".to_string(),
            window_width: 1280.0,
            window_height: 720.0,
            update_frequency: 60,
        }
    }
}

#[derive(Debug)]
pub struct EngineGGRSConfig {}
impl Config for EngineGGRSConfig {
    type Input = GameInput;
    type State = u8;
    type Address = SocketAddr;
}

pub trait EngineApp {
    fn insert_engine(&mut self, config: EngineConfig) -> &mut Self;
}
impl EngineApp for App {
    fn insert_engine(&mut self, config: EngineConfig) -> &mut Self {
        GGRSPlugin::<EngineGGRSConfig>::new()
            // ggrs
            .with_input_system(game_input_system)
            .with_update_frequency(config.update_frequency)
            // components
            .register_rollback_type::<Transform2>()
            .register_rollback_type::<GravityRes>()
            .register_rollback_type::<JointSetRes>()
            .register_rollback_type::<CCDSolverRes>()
            .register_rollback_type::<KinematicBody>()
            .register_rollback_type::<BroadPhaseRes>()
            .register_rollback_type::<PhysicsHandle>()
            .register_rollback_type::<ColliderSetRes>()
            .register_rollback_type::<NarrowPhaseRes>()
            .register_rollback_type::<RigidBodySetRes>()
            .register_rollback_type::<PhysicsCollider>()
            .register_rollback_type::<IslandManagerRes>()
            .register_rollback_type::<QueryPipelineRes>()
            .register_rollback_type::<IntegrationParametersRes>()
            .register_rollback_type::<PhysicsHandleRemovedEntitiesRes>()
            // rollback scheduler
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(RollbackStages::Game, game_scheduler())
                    .with_stage_after(
                        RollbackStages::Game,
                        RollbackStages::Physics,
                        SystemStage::single_threaded()
                            .with_system(physics_system_add)
                            .with_system(physics_system_step)
                            .with_system(physics_system_remove)
                            .with_system(physics_system_kinematic),
                    )
                    .with_stage_after(
                        RollbackStages::Physics,
                        RollbackStages::TransformSynchronization,
                        SystemStage::parallel().with_system(sync_transform_system),
                    ),
            )
            //
            .build(self);

        self
            // plugin
            .add_plugins(DefaultPlugins)
            .add_plugin(ShapePlugin)
            // window
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(WindowDescriptor {
                title: config.window_title.to_owned(),
                vsync: true,
                width: config.window_width,
                height: config.window_height,
                ..Default::default()
            })
            // resources
            .insert_resource(GravityRes::default())
            .insert_resource(JointSetRes::default())
            .insert_resource(CCDSolverRes::default())
            .insert_resource(BroadPhaseRes::default())
            .insert_resource(ColliderSetRes::default())
            .insert_resource(NarrowPhaseRes::default())
            .insert_resource(RigidBodySetRes::default())
            .insert_resource(IslandManagerRes::default())
            .insert_resource(QueryPipelineRes::default())
            .insert_resource(IntegrationParametersRes::default())
            .insert_resource(PhysicsHandleRemovedEntitiesRes::default())
            // startup systems
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new().with_system(physics_startup_system_kinematic_register),
            )
    }
}
