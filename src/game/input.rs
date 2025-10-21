use crate::direction::Direction8;
use crate::game::character_state::CharacterState;

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
