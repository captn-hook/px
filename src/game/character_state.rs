use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CharacterState {
    Still,
    Moving,
}