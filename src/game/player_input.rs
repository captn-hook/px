use bevy::prelude::*;
use crate::game::character_input::CharacterInput;

#[derive(Resource, Clone)]
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
    mut query: Query<&mut PlayerControl>,
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

    for mut control in query.iter_mut() {
        control.player_input = input.clone();
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