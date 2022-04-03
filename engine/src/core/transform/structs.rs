use bevy::prelude::Component;
use bevy::reflect::Reflect;

use crate::core::maths::{Number, Vector2};

#[derive(Default, Reflect, Component)]
pub struct Transform2 {
    pub pos: Vector2,
    pub scale: Vector2,
    pub rotation: Number,
}

impl Transform2 {
    pub fn from_pos(pos: Vector2) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }
    pub fn from_scale(scale: Vector2) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
    pub fn from_rotation(rotation: Number) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }
}
