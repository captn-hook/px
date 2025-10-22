use crate::direction::Direction8;
use crate::game::character_state::CharacterState;
use crate::game::player_input::PlayerControl;
use crate::game::character_input::RandomInput;
use crate::game::character_input::CharacterInput;
use crate::rendering::sprite_set::{get_textures, parse_grid_from_filename};
use crate::rendering::sprite_state::{SpriteBundle, SpriteState};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub direction: Direction8,
    pub transform: Transform,
    pub character_state: CharacterState,
    pub character_input: CharacterInput,
    pub sprite_state: SpriteState,
    pub visibility: Visibility,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            direction: Direction8::East,
            transform: Transform::default(),
            character_state: CharacterState::Still,
            character_input: CharacterInput::default(),
            sprite_state: SpriteState::Still,
            visibility: Visibility::Hidden,
        }
    }
}

pub fn make_children(
    files: Vec<String>,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Vec<SpriteBundle> {
    let mut sprites = Vec::new();

    for filename in files {
        if let Some(grid) = parse_grid_from_filename(&filename) {
            let texture = asset_server.load(&filename);
            let layout = TextureAtlasLayout::from_grid(
                grid.size,
                grid.sprites[0],
                grid.sprites[1],
                None,
                None,
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let sprite_bundle = SpriteBundle::create(texture, texture_atlas_layout, &grid);
            sprites.push(sprite_bundle);
        }
    }

    sprites
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let filenames = get_textures("test_char");
    let children = make_children(filenames, &asset_server, &mut texture_atlas_layouts);

    let parent = commands
        .spawn((
            CharacterBundle::default(),
            PlayerControl::default(),
        )).id();

    for child in children {
        commands.entity(parent).with_children(|parent| {
            parent.spawn(child);
        });
    }

    // translate the player to the left
    commands.entity(parent).insert(Transform::from_translation(Vec3::new(-260.0, 0.0, 0.0)));
}

pub fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let filenames = get_textures("test_char");
    let children = make_children(filenames, &asset_server, &mut texture_atlas_layouts);

    let parent = commands.spawn((
        CharacterBundle::default(),
        RandomInput::default()
    )).id();

    for child in children {
        commands.entity(parent).with_children(|parent| {
            parent.spawn(child);
        });
    }

    // translate the character to the right
    commands.entity(parent).insert(Transform::from_translation(Vec3::new(260.0, 0.0, 0.0)));
}
