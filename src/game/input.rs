use bevy::prelude::*;
// window types will be referenced directly where needed
use crate::direction::Direction8;
use crate::game::character_state::{CharacterState, CurrentCharacterState};
use crate::rendering::sprite_state::Player;
pub use crate::game::player_input::PlayerInput;

/// Adds isolated directional input and derived movement state systems.
pub fn add_direction_input_systems(app: &mut App) {
    // update the global PlayerInput resource from raw input, then derive movement state
    app.add_systems(
        Update,
        (
            update_player_input_keyboard,
            update_player_input_cursor,
            handle_directional_input,
            log_player_input,
        ),
    );
}

/// Read keyboard state into the global PlayerInput resource.
fn update_player_input_keyboard(
    mut input: ResMut<PlayerInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    use KeyCode::*;

    input.up = keyboard.pressed(ArrowUp);
    input.down = keyboard.pressed(ArrowDown);
    input.left = keyboard.pressed(ArrowLeft);
    input.right = keyboard.pressed(ArrowRight);

    // button mappings (change as needed)
    input.action1 = keyboard.pressed(Space);
    input.action2 = keyboard.pressed(ShiftLeft);

    // mouse buttons
    input.click_l = mouse.pressed(MouseButton::Left);
    input.click_r = mouse.pressed(MouseButton::Right);
}

/// Simple debug logger to print the current `PlayerInput` each frame.
fn log_player_input(input: Res<PlayerInput>) {
    // Print a concise one-line summary
    println!(
        "PlayerInput: up={} down={} left={} right={} action1={} action2={} click_l={} click_r={} pointer={:?}",
        input.up,
        input.down,
        input.left,
        input.right,
        input.action1,
        input.action2,
        input.click_l,
        input.click_r,
        input.pointer,
    );
}

/// Read the latest cursor position into the PlayerInput.pointer (window coordinates).
fn update_player_input_cursor(mut input: ResMut<PlayerInput>, windows: Query<&Window>) {
    // read cursor position from the primary window (if any)
    if let Ok(window) = windows.single() {
        if let Some(pos) = window.cursor_position() {
            input.pointer = pos;
        }
    }
}

/// Determine the current direction and movement state from the global PlayerInput resource.
fn handle_directional_input(
    input: Res<PlayerInput>,
    mut query: Query<(&mut Direction8, &mut CurrentCharacterState), With<Player>>,
) {
    // read the single player entity
    if let Ok((mut dir, mut state)) = query.single_mut() {
        let up = input.up;
        let down = input.down;
        let left = input.left;
        let right = input.right;

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