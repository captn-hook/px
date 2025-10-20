use bevy::prelude::*;
use crate::direction::{Direction8, CurrentDirection};
use crate::sprite_state::{SpriteState, CurrentSpriteState};
use crate::rendering::sprite_render::AnimationConfig;

pub fn add_direction_input_systems(app: &mut App) {
    app.add_systems(Update, (handle_direction_input, handle_state_input));
}

fn handle_direction_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current: ResMut<CurrentDirection>,
    mut query: Query<(&Direction8, &SpriteState, &mut AnimationConfig)>,
) {
    let bindings = Direction8::key_bindings();
    for (dir, key) in bindings {
        if keyboard_input.pressed(key) {
            if current.direction != dir {
                current.direction = dir;
                for (sprite_dir, _, mut anim) in &mut query {
                    if *sprite_dir == dir {
                        anim.frame_timer = AnimationConfig::timer_from_fps(anim.fps);
                    }
                }
            }
            return;
        }
    }
}

fn handle_state_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_state: ResMut<CurrentSpriteState>,
    mut query: Query<(&Direction8, &SpriteState, &mut AnimationConfig)>,
) {
    let bindings = SpriteState::key_bindings();
    for (state, key) in bindings {
        if keyboard_input.just_pressed(key) {
            if current_state.state != state {
                current_state.state = state;
                for (_, sprite_state, mut anim) in &mut query {
                    if *sprite_state == state {
                        anim.frame_timer = AnimationConfig::timer_from_fps(anim.fps);
                    }
                }
            }
            return;
        }
    }
}
