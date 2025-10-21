use bevy::prelude::*;
use crate::game::character_input::CharacterInput;
use crate::game::character_state::CharacterState;
use crate::game::input::directional_input;
use crate::direction::Direction8;

#[derive(Resource)]
pub struct PlayerInput {
    pub input: CharacterInput,
    pub click_l: bool,
    pub click_r: bool,
    pub pointer: Vec2,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            input: CharacterInput::default(),
            click_l: false,
            click_r: false,
            pointer: Vec2::ZERO,
        }
    }
}

/// Read keyboard state into the global PlayerInput resource.
pub fn update_player_input(
    mut input: ResMut<PlayerInput>,
    windows: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    use KeyCode::*;

    // movement is true if either arrow keys OR WASD are pressed
    input.input.up = keyboard.pressed(ArrowUp) || keyboard.pressed(KeyCode::KeyW);
    input.input.down = keyboard.pressed(ArrowDown) || keyboard.pressed(KeyCode::KeyS);
    input.input.left = keyboard.pressed(ArrowLeft) || keyboard.pressed(KeyCode::KeyA);
    input.input.right = keyboard.pressed(ArrowRight) || keyboard.pressed(KeyCode::KeyD);

    // button mappings (change as needed)
    input.input.action1 = keyboard.pressed(Space);
    input.input.action2 = keyboard.pressed(ShiftLeft);

    // mouse buttons
    input.click_l = mouse.pressed(MouseButton::Left);
    input.click_r = mouse.pressed(MouseButton::Right);
    
    if let Ok(window) = windows.single() {
        if let Some(pos) = window.cursor_position() {
            input.pointer = pos;
        }
    }
}

#[derive(Component)]
pub struct PlayerControl {
    pub player_input: PlayerInput,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            player_input: PlayerInput::default(),
        }
    }
}

// Player control system just reads the PlayerInput resource and applies it to the entity
// for entity movement, there must be a Transform, Direction8, and CharacterState components.
// transform not implemented yet
pub fn player_control_system(
    mut query: Query<(&mut Direction8, &mut CharacterState, &PlayerControl)>) {
    for (mut direction, mut state, player_control) in query.iter_mut() {
        let input = &player_control.player_input.input;
        let (new_direction, new_state) = directional_input([
            input.up,
            input.down,
            input.left,
            input.right,
        ]);

        if new_direction.is_some() {
            *direction = new_direction.unwrap();
        }
        *state = new_state;

        // Apply the input to the transform or other components as needed
        // Not implemented yet, we dont want to modify transform directly without some checks
    }
}