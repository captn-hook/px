use bevy::prelude::*;
use crate::direction::{Direction8, CurrentDirection};
use crate::game::character_state::{CharacterState, CurrentCharacterState};

/// Adds isolated directional input and derived movement state systems.
pub fn add_direction_input_systems(app: &mut App) {
    app.add_systems(Update, handle_directional_input);
}

/// Determine the current direction and movement state from arrow key input.
fn handle_directional_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_dir: ResMut<CurrentDirection>,
    mut current_state: ResMut<CurrentCharacterState>,
) {
    use KeyCode::*;

    // --- 1. Determine pressed direction vector ---
    let up = keyboard.pressed(ArrowUp);
    let down = keyboard.pressed(ArrowDown);
    let left = keyboard.pressed(ArrowLeft);
    let right = keyboard.pressed(ArrowRight);

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

    let prev_dir = current_dir.direction;

    // --- 3. Apply new direction if changed ---
    if let Some(dir) = new_direction {
        if dir != prev_dir {
            current_dir.direction = dir;
        }
    }

    current_state.state = if new_direction.is_some() {
        CharacterState::Moving
    } else {
        CharacterState::Still
    };
}