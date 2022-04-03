use bevy::reflect::Reflect;
use derive_more::*;
use fixed::traits::ToFixed;

use crate::core::maths::Number;

// Reflect
#[derive(Copy, Clone, Default, Reflect)]
// Comparison
#[derive(Eq, PartialOrd, Ord, PartialEq)]
// Math operators
#[derive(Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign, Neg)]
pub struct Vector2 {
    pub x: Number,
    pub y: Number,
}

impl Vector2 {
    pub fn new<T: ToFixed>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl std::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2")
            .field("x", &f32::from(self.x))
            .field("y", &f32::from(self.y))
            .finish()
    }
}
