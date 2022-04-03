use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;

#[repr(C)]
#[derive(Pod, Copy, Clone, PartialEq, Zeroable)]
pub struct GameInput {
    pub mask: u8,
}

pub fn game_input_system(_: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> GameInput {
    let mut mask: u8 = 0;

    if keyboard_input.pressed(KeyCode::Up) {
        mask |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        mask |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        mask |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        mask |= INPUT_RIGHT;
    }

    GameInput { mask }
}
