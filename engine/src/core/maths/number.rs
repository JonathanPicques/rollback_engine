use bevy::reflect::impl_reflect_value;
use bevy::reflect::ReflectDeserialize;
use derive_more::*;
use fixed::traits::ToFixed;
use fixed::types::extra::U4;
use fixed::FixedI32;
use serde::{Deserialize, Serialize};

type FixedImpl = FixedI32<U4>;

// Reflect
#[derive(Copy, Clone, Default, Serialize, Deserialize)]
// Comparison
#[derive(Eq, PartialOrd, Ord, PartialEq)]
// Math operators
#[derive(Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign, Neg)]
#[mul(forward)]
pub struct Number(pub FixedImpl);

impl_reflect_value!(Number(Serialize, Deserialize));

impl From<Number> for f32 {
    fn from(val: Number) -> Self {
        FixedImpl::to_num(val.0)
    }
}
impl From<Number> for i32 {
    fn from(val: Number) -> Self {
        FixedImpl::to_num(val.0)
    }
}
impl From<Number> for u32 {
    fn from(val: Number) -> Self {
        FixedImpl::to_num(val.0)
    }
}
impl From<Number> for usize {
    fn from(val: Number) -> Self {
        FixedImpl::to_num(val.0)
    }
}
impl<Src: ToFixed> From<Src> for Number {
    fn from(src: Src) -> Self {
        Self(FixedImpl::from_num(src))
    }
}
