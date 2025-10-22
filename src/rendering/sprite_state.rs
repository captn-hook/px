use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Sequence)]
pub enum SpriteState {
    Still,
    Starting,
    Moving,
    Stopping,
}

// to string
impl SpriteState {
    pub fn as_str(&self) -> &str {
        match self {
            SpriteState::Still => "still",
            SpriteState::Starting => "starting",
            SpriteState::Moving => "moving",
            SpriteState::Stopping => "stopping",
        }
    }
}