use bevy::prelude::*;
use std::collections::HashMap;

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

    pub fn key_bindings() -> HashMap<Direction8, KeyCode> {
        use Direction8::*;
        use KeyCode::*;
        HashMap::from([
            (North, ArrowUp),
            (Northeast, KeyW),
            (East, ArrowRight),
            (Southeast, KeyD),
            (South, ArrowDown),
            (Southwest, KeyS),
            (West, ArrowLeft),
            (Northwest, KeyA),
        ])
    }
}

#[derive(Resource)]
pub struct CurrentDirection {
    pub direction: Direction8,
}

impl CurrentDirection {
    pub fn new(direction: Direction8) -> Self {
        Self { direction }
    }
}