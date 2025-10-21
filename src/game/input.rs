use bevy::prelude::*;
use crate::direction::Direction8;
use crate::game::character_state::{CharacterState, CurrentCharacterState};
use crate::rendering::sprite_state::Player;

/// Adds isolated directional input and derived movement state systems.
pub fn add_direction_input_systems(app: &mut App) {
    app.add_systems(Update, handle_directional_input);
}

/// Determine the current direction and movement state from arrow key input.
fn handle_directional_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Direction8, &mut CurrentCharacterState), With<Player>>,
) {
    use KeyCode::*;
    // read the single player entity
    if let Ok((mut dir, mut state)) = query.single_mut() {
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

        if let Some(d) = new_direction {
            *dir = d;
        }

        state.state = if new_direction.is_some() {
            CharacterState::Moving
        } else {
            CharacterState::Still
        };
    }
}