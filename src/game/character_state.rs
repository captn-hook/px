use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CharacterState {
    Still,
    Moving,
}

impl CharacterState {
    pub fn all() -> &'static [CharacterState] {
        use CharacterState::*;
        &[Still, Moving]
    }
}