use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action1: bool,
    pub action2: bool,
    pub click_l: bool,
    pub click_r: bool,
    pub pointer: Vec2,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            action1: false,
            action2: false,
            click_l: false,
            click_r: false,
            pointer: Vec2::ZERO,
        }
    }
}
