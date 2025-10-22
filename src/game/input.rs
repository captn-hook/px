use bevy::prelude::*;
use crate::direction::Direction8;
use crate::game::character_state::CharacterState;
use crate::game::character_input::CharacterInput;

pub fn update_characters(
    mut query: Query<(&mut Direction8, &mut CharacterState, &mut Transform, &CharacterInput)>,
) {
    // get player and npc inputs here if needed
    for (mut direction, mut state, mut transform, input) in query.iter_mut() {
        let (new_direction, new_state) = directional_input(input.as_array()[0..4].try_into().unwrap());

        if let Some(dir) = new_direction {
            *direction = dir;
        }
        *state = new_state;

        // Simple movement test
        if *state == CharacterState::Moving {
            transform.translation += direction.to_translation().extend(0.0) * 0.17;
        }
    }
}

    
/// Converts keys to dpad style movement and state
pub fn directional_input(input: [bool; 4]) -> (Option<Direction8>, CharacterState) {
    let up = input[0];
    let down = input[1];
    let left = input[2];
    let right = input[3];

    let new_direction = match (up, down, left, right) {
        (true, false, false, false) => Some(Direction8::North),
        (true, false, true, false) => Some(Direction8::Northwest),
        (true, false, false, true) => Some(Direction8::Northeast),
        (false, true, false, false) => Some(Direction8::South),
        (false, true, true, false) => Some(Direction8::Southwest),
        (false, true, false, true) => Some(Direction8::Southeast),
        (false, false, true, false) => Some(Direction8::West),
        (false, false, false, true) => Some(Direction8::East),
        _ => None,
    };

    let new_state = if new_direction.is_some() {
        CharacterState::Moving
    } else {
        CharacterState::Still
    };

    (new_direction, new_state)
}
