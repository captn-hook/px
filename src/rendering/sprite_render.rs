use crate::direction::Direction8;
use crate::spawn::AnimationTimer;
use crate::game::character_state::CharacterState;
use crate::rendering::sprite_set::{SpriteAndIndices, SpriteSet};
use crate::rendering::sprite_state::SpriteState;
use bevy::prelude::*;
use enum_iterator::all;
use std::collections::HashMap;
use std::path::Path;


pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn render_sprites(
    // Query for all entities with a SpriteSet, SpriteState, CharacterState, Direction8, and Transform
    mut query: Query<(
        &mut SpriteSet,
        &mut SpriteState,
        &mut AnimationTimer,
        &CharacterState,
        &Direction8,
    )>,
    time: Res<Time>,
) {
    // for (set_name, set) in sprite_library.sets.iter_mut() {
    //     // println!("Drawing sprite set: {}", set_name);
    //     set.draw(time.delta());
    // }

    for (mut sprite_set, mut sprite_state, mut animation_timer, character_state, direction) in
        query.iter_mut()
    {
        // Update the sprite state based on the character state, using transitions from one character state to another
        match character_state {
            CharacterState::Still => match *sprite_state {
                SpriteState::Moving => {
                    *sprite_state = SpriteState::Stopping;
                }
                SpriteState::Stopping => {
                    *sprite_state = SpriteState::Still;
                }
                _ => {
                    *sprite_state = SpriteState::Still;
                }
            },
            CharacterState::Moving => match *sprite_state {
                SpriteState::Still => {
                    *sprite_state = SpriteState::Starting;
                }
                SpriteState::Starting => {
                    *sprite_state = SpriteState::Moving;
                }
                _ => {
                    *sprite_state = SpriteState::Moving;
                }
            },
        }

        animation_timer.tick(time.delta());

        if animation_timer.just_finished() {
            if let Some(sprite) = sprite_set.get_sprite(*direction, *sprite_state) {
                if let Some(atlas) = &mut sprite.sprite.texture_atlas {
                    atlas.index = if atlas.index == sprite.last_index {
                        sprite.first_index
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

pub fn load_spriteset(
    name: &str,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> HashMap<String, SpriteAndIndices> {
    let mut map: HashMap<String, SpriteAndIndices> = HashMap::new();
    for direction in all::<Direction8>() {
        for state in all::<SpriteState>() {
            // get correct file name for this sprite
            let dir_state = format!("{}_{}", direction.as_str(), state.as_str());
            let file = find_existing_texture(name, &dir_state);
            if let Some(file) = file {
                let grid = parse_grid_from_filename(&file);
                if let Some(grid) = grid {
                    let image_handle: Handle<Image> = asset_server.load(&file);

                    let sprite = make_sprite(image_handle, &mut texture_atlas_layouts, grid);

                    map.insert(dir_state, sprite);
                }
            }
        }
    }

    return map;
}

pub fn make_sprite(
    image_handle: Handle<Image>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: Grid,
) -> SpriteAndIndices {
    let layout =
        TextureAtlasLayout::from_grid(grid.size, grid.sprites[0], grid.sprites[1], None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    // let sprite = Sprite {
    //     image: image_handle,
    //     texture_atlas: Some(TextureAtlas {
    //         layout: layout_handle,
    //         index: 0,
    //     }),
    //     ..default()
    // };
    let sprite = Sprite::from_atlas_image(
        image_handle,
        TextureAtlas {
            layout: layout_handle,
            index: grid.sprites[0] as usize,
        },
    );

    return SpriteAndIndices {
        sprite,
        first_index: grid.sprites[0] as usize,
        last_index: grid.sprites[1] as usize,
    };
}

/// Checks if "base_NxM.png" exists.
fn find_existing_texture(set: &str, base: &str) -> Option<String> {
    let texture_dir = Path::new("assets");

    // check for any "_NxM.png"
    if let Ok(entries) = std::fs::read_dir(texture_dir.join(format!("textures/{}", set))) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&base) && name.ends_with(".png") {
                    return Some(format!("textures/{}/{}", set, name));
                }
            }
        }
    }
    return None;
}

#[derive(Debug)]
pub struct Grid {
    pub sprites: UVec2,
    pub size: UVec2,
}

pub fn parse_grid_from_filename(filename: &str) -> Option<Grid> {
    // Files are named as "base_AxB_CxD.png"
    // AxB is the number of sprites in the sheet, as in there are A columns and B rows of frames.
    // CxD is the size of each sprite in the sheet, as in each sprite is C pixels wide and D pixels tall.

    let stem = Path::new(filename).file_stem()?.to_str()?;

    // Split into parts using '_'
    let parts: Vec<&str> = stem.split('_').collect();

    // Check if we have at least three parts (the grid and sprite size)
    if parts.len() >= 3 {
        let sprite_grid = parts[parts.len() - 2]; // 2x5 or similar
        let sprite_size = parts[parts.len() - 1]; // 500x500 or similar

        // Parse the grid size (2x5 -> 2 columns, 5 rows)
        if let Some((cols_str, rows_str)) = sprite_grid.split_once('x') {
            let cols = cols_str.parse().ok()?;
            let rows = rows_str.parse().ok()?;

            // Parse the sprite size (500x500 -> 500 width, 500 height)
            if let Some((width_str, height_str)) = sprite_size.split_once('x') {
                let width = width_str.parse().ok()?;
                let height = height_str.parse().ok()?;

                return Some(Grid {
                    sprites: UVec2::new(cols, rows),
                    size: UVec2::new(width, height),
                });
            }
        }
    }

    None
}
