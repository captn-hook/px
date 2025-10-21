use crate::direction::Direction8;
use crate::game::character_input::RandomInput;
use crate::game::player_input::PlayerControl;
use crate::game::character_state::CharacterState;
use crate::rendering::sprite_set::SpriteSet;
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

pub fn spawn_characters(mut commands: Commands) {
    commands.spawn((
        CharacterBundle {
            direction: Direction8::South,
            sprite_state: SpriteState::Still,
            character_state: CharacterState::Still,
            sprite_set: SpriteSet::new("test_char"),
            transform: Transform::from_translation(Vec3::Y * 2.0),
        },
        PlayerControl::default(),
    ));
    commands.spawn((
        CharacterBundle {
            direction: Direction8::South,
            sprite_state: SpriteState::Still,
            character_state: CharacterState::Still,
            sprite_set: SpriteSet::new("test_char"),
            transform: Transform::from_translation(Vec3::X * 2.0),
        },
        RandomInput::default(),
    ));
}
