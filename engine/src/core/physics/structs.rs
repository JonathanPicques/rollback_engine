use bevy::prelude::{Bundle, Component, Entity};
use bevy::reflect::{impl_reflect_value, Reflect, ReflectDeserialize};
use derive_more::{Deref, DerefMut};
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::maths::Vector2;

// Physics state resources

#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct GravityRes(pub Vector<f32>);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct JointSetRes(pub JointSet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct CCDSolverRes(pub CCDSolver);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct BroadPhaseRes(pub BroadPhase);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct ColliderSetRes(pub ColliderSet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct NarrowPhaseRes(pub NarrowPhase);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct RigidBodySetRes(pub RigidBodySet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct IslandManagerRes(pub IslandManager);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct QueryPipelineRes(pub QueryPipeline);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct IntegrationParametersRes(pub IntegrationParameters);

impl Default for GravityRes {
    fn default() -> Self {
        Self(vector![0.0, -9.81])
    }
}
impl Default for JointSetRes {
    fn default() -> Self {
        Self(JointSet::new())
    }
}
impl Default for CCDSolverRes {
    fn default() -> Self {
        Self(CCDSolver::new())
    }
}
impl Default for BroadPhaseRes {
    fn default() -> Self {
        Self(BroadPhase::new())
    }
}
impl Default for ColliderSetRes {
    fn default() -> Self {
        Self(ColliderSet::new())
    }
}
impl Default for NarrowPhaseRes {
    fn default() -> Self {
        Self(NarrowPhase::new())
    }
}
impl Default for RigidBodySetRes {
    fn default() -> Self {
        Self(RigidBodySet::new())
    }
}
impl Default for IslandManagerRes {
    fn default() -> Self {
        Self(IslandManager::new())
    }
}
impl Default for QueryPipelineRes {
    fn default() -> Self {
        Self(QueryPipeline::default())
    }
}
impl Default for IntegrationParametersRes {
    fn default() -> Self {
        Self(IntegrationParameters::default())
    }
}
impl_reflect_value!(GravityRes(Serialize, Deserialize));
impl_reflect_value!(JointSetRes(Serialize, Deserialize));
impl_reflect_value!(CCDSolverRes(Serialize, Deserialize));
impl_reflect_value!(BroadPhaseRes(Serialize, Deserialize));
impl_reflect_value!(ColliderSetRes(Serialize, Deserialize));
impl_reflect_value!(NarrowPhaseRes(Serialize, Deserialize));
impl_reflect_value!(RigidBodySetRes(Serialize, Deserialize));
impl_reflect_value!(IslandManagerRes(Serialize, Deserialize));
impl_reflect_value!(QueryPipelineRes(Serialize, Deserialize));
impl_reflect_value!(IntegrationParametersRes(Serialize, Deserialize));

// Physics ECS components

#[derive(Clone, Debug, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct PhysicsHandle(pub RigidBodyHandle);

impl Default for PhysicsHandle {
    fn default() -> Self {
        Self(RigidBodyHandle::invalid())
    }
}
impl_reflect_value!(PhysicsHandle(Serialize, Deserialize));

#[derive(Clone, Default, Reflect, Component)]
pub struct PhysicsCollider {
    pub size: Vector2,
    pub layer: u32,
    pub layer_mask: u32,
}

// Physics ECS bodies components

#[derive(Clone, Default, Reflect, Component)]
pub struct StaticBody {}

#[derive(Clone, Default, Reflect, Component)]
pub struct KinematicBody {
    pub velocity: Vector2,
    pub(crate) is_on_wall: bool,
    pub(crate) is_on_floor: bool,
    pub(crate) is_on_ceiling: bool,
}

#[derive(Bundle)]
pub struct StaticBodyBundle {
    pub body: StaticBody,
    pub handle: PhysicsHandle,
    pub collider: PhysicsCollider,
}

#[derive(Default, Bundle)]
pub struct KinematicBodyBundle {
    pub body: KinematicBody,
    pub handle: PhysicsHandle,
    pub collider: PhysicsCollider,
}

// Physics ECS components book-keeping

#[derive(Clone, Default, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct PhysicsHandleRemovedEntitiesRes(pub HashMap<Entity, RigidBodyHandle>);

impl_reflect_value!(PhysicsHandleRemovedEntitiesRes(Serialize, Deserialize));
