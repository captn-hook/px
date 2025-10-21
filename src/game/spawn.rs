use bevy::prelude::*;
use crate::direction::Direction8;
use crate::rendering::sprite_state::{Player, SpriteState};
use crate::game::character_state::CurrentCharacterState;

pub fn spawn_player(mut commands: Commands) {
	commands.spawn((
		Player,
		Direction8::South,
		SpriteState::Still,
		CurrentCharacterState::default(),
		Transform::default(),
		GlobalTransform::default(),
	));
}
