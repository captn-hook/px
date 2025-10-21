use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum SpriteState {
    Still,
    Starting,
    Moving,
    Stopping,
}

impl SpriteState {
    pub fn all() -> &'static [SpriteState] {
        use SpriteState::*;
        &[Still, Starting, Moving, Stopping]
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

impl Default for CurrentSpriteState {
    fn default() -> Self {
        Self {
            state: SpriteState::Still,
        }
    }
}