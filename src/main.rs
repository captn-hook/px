use bevy::prelude::*;
mod direction;
mod spawn;
mod rendering;
mod game;

use rendering::sprite_set::{SpriteLibrary, SpriteSet};
use rendering::sprite_render::{setup, process_atlases, execute_animations, update_visibility};

use game::player_input::{PlayerControl, PlayerInput, update_player_input};
use game::character_input::{RandomInput, update_random_input};

use spawn::spawn_characters;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(PlayerInput::default()) // global keyboard + mouse input
        .insert_resource(SpriteLibrary::default()) // global sprite library
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_characters)
        .add_systems(Update, (update_player_input, update_random_input))
        .add_systems(Update, (process_atlases, execute_animations, update_visibility));

    app.run();
}
