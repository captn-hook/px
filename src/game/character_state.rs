use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CharacterState {
    Still,
    Moving,
}

#[derive(Component)]
pub struct CurrentCharacterState {
    pub state: CharacterState,
}

impl CurrentCharacterState {
}

impl Default for CurrentCharacterState {
    fn default() -> Self {
        Self {
            state: CharacterState::Still,
        }
    }
}