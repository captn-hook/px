use crate::direction::Direction8;
use crate::game::character_input::RandomInput;
use crate::game::character_state::CharacterState;
use crate::game::player_input::PlayerControl;
use crate::rendering::sprite_set::{SpriteLibrary, SpriteSet};
use crate::rendering::sprite_state::SpriteState;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub direction: Direction8,
    pub sprite_state: SpriteState,
    pub character_state: CharacterState,
    pub sprite_set: SpriteSet,
    pub transform: Transform,
}

pub fn spawn_player(mut commands: Commands, sprite_library: Res<SpriteLibrary>) {
    commands.spawn((
        CharacterBundle {
            direction: Direction8::South,
            sprite_state: SpriteState::Still,
            character_state: CharacterState::Still,
            sprite_set: SpriteSet::default(sprite_library),
            transform: Transform::from_translation(Vec3::Y * 2.0),
        },
        PlayerControl::default(),
    ));
}

pub fn spawn_character(mut commands: Commands, sprite_library: Res<SpriteLibrary>) {
    commands.spawn((
        CharacterBundle {
            direction: Direction8::South,
            sprite_state: SpriteState::Still,
            character_state: CharacterState::Still,
            sprite_set: SpriteSet::default(sprite_library),
            transform: Transform::from_translation(Vec3::X * 2.0),
        },
        RandomInput::default(),
    ));
}
