use bevy::prelude::*;
mod direction;
mod sprite_state;
mod rendering;
mod game;

use direction::{CurrentDirection, Direction8};
use sprite_state::{CurrentSpriteState, SpriteState};
use rendering::sprite_render::{setup, process_atlases, execute_animations, update_visibility};
use game::input::add_direction_input_systems;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(CurrentDirection::new(Direction8::South))
        .insert_resource(CurrentSpriteState::new(SpriteState::Still))
        .add_systems(Startup, setup)
        .add_systems(Update, (process_atlases, execute_animations, update_visibility));

    add_direction_input_systems(&mut app);

    app.run();
}
