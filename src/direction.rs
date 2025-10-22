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

    pub fn from_str(s: &str) -> Option<Direction8> {
        match s {
            "north" => Some(Direction8::North),
            "northeast" => Some(Direction8::Northeast),
            "east" => Some(Direction8::East),
            "southeast" => Some(Direction8::Southeast),
            "south" => Some(Direction8::South),
            "southwest" => Some(Direction8::Southwest),
            "west" => Some(Direction8::West),
            "northwest" => Some(Direction8::Northwest),
            _ => None,
        }
    }

    pub fn to_translation(&self) -> Vec2 {
        match self {
            Direction8::North => Vec2::new(0.0, 1.0),
            Direction8::Northeast => Vec2::new(1.0, 1.0).normalize(),
            Direction8::East => Vec2::new(1.0, 0.0),
            Direction8::Southeast => Vec2::new(1.0, -1.0).normalize(),
            Direction8::South => Vec2::new(0.0, -1.0),
            Direction8::Southwest => Vec2::new(-1.0, -1.0).normalize(),
            Direction8::West => Vec2::new(-1.0, 0.0),
            Direction8::Northwest => Vec2::new(-1.0, 1.0).normalize(),
        }
    }
}