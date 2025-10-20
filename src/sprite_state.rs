use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum SpriteState {
    Still,
    Moving,
    Blocked,
    Attacking,
}

impl SpriteState {
    pub fn all() -> &'static [SpriteState] {
        use SpriteState::*;
        &[Still, Moving, Blocked, Attacking]
    }

    pub fn key_bindings() -> HashMap<SpriteState, KeyCode> {
        use SpriteState::*;
        use KeyCode::*;
        HashMap::from([
            (Still, KeyZ),
            (Moving, KeyX)
        ])
    }
}

#[derive(Resource)]
pub struct CurrentSpriteState {
    pub state: SpriteState,
}

impl CurrentSpriteState {
    pub fn new(state: SpriteState) -> Self {
        Self { state }
    }
}