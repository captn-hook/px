use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Sequence)]
pub enum Direction8 {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl Direction8 {
    pub fn as_str(&self) -> &str {
        match self {
            Direction8::North => "north",
            Direction8::Northeast => "northeast",
            Direction8::East => "east",
            Direction8::Southeast => "southeast",
            Direction8::South => "south",
            Direction8::Southwest => "southwest",
            Direction8::West => "west",
            Direction8::Northwest => "northwest",
        }
    }
}