use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CharacterState {
    Still,
    Moving,
}

#[derive(Resource)]
pub struct CurrentCharacterState {
    pub state: CharacterState,
}

impl CurrentCharacterState {
    pub fn new(state: CharacterState) -> Self {
        Self { state }
    }
}

impl Default for CurrentCharacterState {
    fn default() -> Self {
        Self {
            state: CharacterState::Still,
        }
    }
}