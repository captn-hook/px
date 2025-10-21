use bevy::prelude::*;
mod direction;
mod rendering;
mod game;

use rendering::sprite_render::{setup, process_atlases, execute_animations, update_visibility};
use game::input::{add_direction_input_systems, PlayerInput};
use game::spawn::spawn_player;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(PlayerInput::default())    
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (process_atlases, execute_animations, update_visibility));

    add_direction_input_systems(&mut app);

    app.run();
}
