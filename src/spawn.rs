use bevy::prelude::*;
use crate::direction::Direction8;
use crate::rendering::sprite_state::SpriteState;
use crate::game::character_state::CharacterState;
use crate::rendering::sprite_set::SpriteSet;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub direction: Direction8,
    pub sprite_state: SpriteState,
    pub character_state: CharacterState,
    pub sprite_set: SpriteSet,
    pub transform: Transform,
}

pub fn spawn_character<C: Bundle>(
    mut commands: Commands,
    position: Vec3,
	sprite_set: SpriteSet,
    extra: C, // any extra components
) {
    commands.spawn((
        CharacterBundle {
            direction: Direction8::South,
            sprite_state: SpriteState::Still,
            character_state: CharacterState::Still,
            sprite_set: sprite_set,
            transform: Transform::from_translation(position),
        },
        extra,
    ));
}