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

// Marker component for the Player entity
#[derive(Component)]
pub struct Player;
