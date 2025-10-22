use std::path::Path;
use bevy::prelude::*;
use crate::rendering::sprite_state::SpriteState;
use crate::direction::Direction8;
use enum_iterator::all;


#[derive(Debug)]
pub struct Grid {
    pub sprites: UVec2,
    pub size: UVec2,
    pub direction: Direction8,
    pub state: SpriteState,
}

/// Gets all available textures for a sprite name
pub fn get_textures(name: &str) -> Vec<String> {
    let mut textures = Vec::new();

    for direction in all::<Direction8>() {
        for state in all::<SpriteState>() {
            let base = format!("{}_{}", direction.as_str(), state.as_str());
            if let Some(texture) = find_existing_texture(name, &base) {
                textures.push(texture);
            }
        }
    }

    textures
}

/// Checks if "base_NxM.png" exists.
pub fn find_existing_texture(set: &str, base: &str) -> Option<String> {
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
        
        if let Some(direction) = Direction8::from_str(parts[0]) {
            if let Some(state) = SpriteState::from_str(parts[1]) {
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
                            direction,
                            state,
                        });
                    }
                }
            }
        }
    }

    None
}