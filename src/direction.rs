use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
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
    pub fn all() -> &'static [Direction8] {
        use Direction8::*;
        &[
            North, Northeast, East, Southeast,
            South, Southwest, West, Northwest,
        ]
    }
}

#[derive(Resource)] // change to component
pub struct CurrentDirection {
    pub direction: Direction8,
}

impl CurrentDirection {
    pub fn new(direction: Direction8) -> Self {
        Self { direction }
    }
}