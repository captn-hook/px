use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Sequence)]
pub enum SpriteState {
    Still,
    Starting,
    Moving,
    Stopping,
}