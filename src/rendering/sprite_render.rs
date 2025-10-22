use crate::direction::Direction8;
use crate::game::character_state::CharacterState;
use crate::rendering::sprite_state::SpriteState;
use crate::rendering::sprite_state::{AnimationIndices, AnimationTimer};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn update_character_sprites(
    mut char_query: Query<(
        &CharacterState,
        &Direction8,
        &mut SpriteState,
        &mut Children,
    )>,
    mut sprite_query: Query<(&SpriteState, &Direction8, &AnimationIndices, &mut Visibility), Without<CharacterState>>,
) {
    // change this to track last state and direction to avoid unnecessary updates
    for (state, direction, mut sprite, children) in char_query.iter_mut() {
        
        let mut can_change = false;

        // Update child sprite visibilities
        for child in children.iter() {
            let (child_sprite_state, child_direction, indices, mut visibility) =
                sprite_query.get_mut(child).unwrap();

            if *child_sprite_state == *sprite && *child_direction == *direction {
                *visibility = Visibility::Visible;
                
                if state == &CharacterState::Still || indices.current == indices.last || indices.current == indices.first {
                    can_change = true;
                }

            } else {
                *visibility = Visibility::Hidden;
            }
        }

                // Update sprite state based on character state and direction
        if can_change {
            match state {
                CharacterState::Still => match *sprite {
                    SpriteState::Moving | SpriteState::Starting => {
                        *sprite = SpriteState::Stopping;
                    }
                    _ => {
                        *sprite = SpriteState::Still;
                    }
                },
                CharacterState::Moving => match *sprite {
                    SpriteState::Still | SpriteState::Stopping => {
                        *sprite = SpriteState::Starting;
                    }
                    _ => {
                        *sprite = SpriteState::Moving;
                    }
                },
            }
        }
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
        &Visibility,
    )>,
) {
    for (mut indices, mut timer, mut sprite, visibility) in query.iter_mut() {
        if *visibility == Visibility::Hidden {
            continue;
        }
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
                indices.current = atlas.index;
            }
        }
    }
}
