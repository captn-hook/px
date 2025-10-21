use bevy::prelude::*;
mod direction;
mod spawn;
mod rendering;
mod game;

use rendering::sprite_set::SpriteLibrary;
use rendering::sprite_render::{setup_camera, render_sprites};

use game::player_input::{PlayerInput, update_player_input};
use game::character_input::update_random_input;

use spawn::{spawn_player, spawn_character};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(PlayerInput::default()) // global keyboard + mouse input
        .insert_resource(SpriteLibrary::default()) // global sprite library
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (spawn_player, spawn_character))
        .add_systems(Update, (update_player_input, update_random_input))
        .add_systems(Update, render_sprites);

    app.run();
}
