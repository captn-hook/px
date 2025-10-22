use crate::rendering::sprite_render::load_spriteset;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::rendering::sprite_state::SpriteState;
use crate::direction::Direction8;
/// A library containing all loaded sprite sets. Each set is keyed by a unique
/// name so different entities can refer to the same shared data by name.
#[derive(Resource)]
pub struct SpriteLibrary {
    /// Mapping from sprite-set name -> SpriteSet
    pub sets: HashMap<String, SpriteSet>,
}

impl Default for SpriteLibrary {
    fn default() -> Self {
        Self {
            sets: HashMap::new(),
        }
    }
}

impl SpriteLibrary {
    pub fn get(&self, name: &str) -> SpriteSet {
        // if we have the sprite set in the library, return it
        if let Some(sprite_set) = self.sets.get(name) {
            return sprite_set.clone();
        } else {
            print!("Sprite set '{}' not found in library", name);
            return SpriteSet::create(HashMap::new());
        }
    }

    pub fn add_sprite_set(
        &mut self,
        name: &str,
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> SpriteSet {
        // create a new sprite set and add it to the library
        let sprite_set = SpriteSet::create(
            load_spriteset(name, asset_server, texture_atlas_layouts),
        );
        self.sets.insert(name.to_string(), sprite_set);
        if let Some(sprite_set) = self.sets.get(name) {
            // check how many sprites were loaded
            return sprite_set.clone();
        }
        panic!("Failed to add sprite set");
    }
}

pub fn load_sprites(
    mut sprite_library: ResMut<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    sprite_library.add_sprite_set(
        "test_char",
        asset_server,
        texture_atlas_layouts,
    );
}

#[derive(Clone)]
pub struct SpriteAndIndices {
    pub sprite: Sprite,
    pub first_index: usize,
    pub last_index: usize,
}

#[derive(Component, Clone)]
pub struct SpriteSet {
    pub atlases: HashMap<String, SpriteAndIndices>,
}

impl SpriteSet {
    pub fn default(sprite_library: Res<SpriteLibrary>) -> SpriteSet {
        let default_set = "test_char";
        let sprite_set = sprite_library.get(default_set);
        return sprite_set;
    }

    pub fn create(spr: HashMap<String, SpriteAndIndices>) -> Self {
        SpriteSet {
            atlases: spr,
        }
    }

    pub fn get_sprite(&mut self, direction: Direction8, state: SpriteState) -> Option<&mut SpriteAndIndices> {
        let dir_state = format!("{}_{}", direction.as_str(), state.as_str());
        return self.atlases.get_mut(&dir_state);
    }
}
