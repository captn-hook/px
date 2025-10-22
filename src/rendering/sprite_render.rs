use crate::direction::Direction8;
use crate::rendering::sprite_set::{SpriteLibrary, SpriteSet};
use crate::rendering::sprite_state::SpriteState;
use bevy::prelude::*;
use enum_iterator::all;
use std::collections::HashMap;
use std::path::Path;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn render_sprites(
    // windows: Query<&Window>, // for sizing (unused for now)
    mut sprite_library: ResMut<SpriteLibrary>,
    mut query: Query<&mut SpriteSet>,
    time: Res<Time>,
) {
    for (set_name, set) in sprite_library.sets.iter_mut() {
        // println!("Drawing sprite set: {}", set_name);
        set.draw(time.delta());
    }

    for mut sprite_set in query.iter_mut() {
        *sprite_set = sprite_library.get(&sprite_set.name);
    }
}

pub fn load_sprite(
    name: &str,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> HashMap<String, Sprite> {
    let mut map: HashMap<String, Sprite> = HashMap::new();
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

                    println!("Loaded sprite for {}: {}", dir_state, file);
                    println!("Sprite info: image handle: {:?}, texture atlas: {:?}", sprite.image, sprite.texture_atlas);

                    map.insert(dir_state, sprite);
                }
            }
        }
    }
    println!("Loaded {} sprites for sprite set '{}'", map.len(), name);

    return map;
}

pub fn make_sprite(
    image_handle: Handle<Image>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    grid: Grid,
) -> Sprite {
    let layout =
        TextureAtlasLayout::from_grid(grid.size, grid.sprites[0], grid.sprites[1], None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    let sprite = Sprite {
        image: image_handle,
        texture_atlas: Some(TextureAtlas {
            layout: layout_handle,
            index: 0,
        }),
        ..default()
    };
    return sprite;
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
    sprites: UVec2,
    size: UVec2,
}

fn parse_grid_from_filename(filename: &str) -> Option<Grid> {
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
