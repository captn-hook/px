use crate::rendering::sprite_render::load_sprite;
use bevy::prelude::*;
use core::time::Duration;
use std::collections::HashMap;

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
    pub fn get(&mut self, name: &str) -> SpriteSet {
        // if we have the sprite set in the library, return it
        if let Some(sprite_set) = self.sets.get(name) {
            return sprite_set.clone();
        } else {
            print!("Sprite set '{}' not found in library", name);
            return SpriteSet::create(name, HashMap::new());
        }
    }

    pub fn add_sprite_set(
        &mut self,
        name: &str,
        asset_server: Res<AssetServer>,
        assets: Res<Assets<Image>>,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> SpriteSet {
        // create a new sprite set and add it to the library
        let sprite_set = SpriteSet::create(
            name,
            load_sprite(name, asset_server, assets, texture_atlas_layouts),
        );
        self.sets.insert(name.to_string(), sprite_set);
        if let Some(sprite_set) = self.sets.get(name) {
            return sprite_set.clone();
        }
        panic!("Failed to add sprite set");
    }
}

pub fn load_sprites(
    mut sprite_library: ResMut<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Loading sprites...");
    sprite_library.add_sprite_set(
        "test_char",
        asset_server,
        assets,
        texture_atlas_layouts,
    );
}

#[derive(Component, Clone)]
pub struct SpriteSet {
    pub name: String,
    pub atlases: HashMap<String, Sprite>,
}

impl SpriteSet {
    pub fn default(sprite_library: ResMut<SpriteLibrary>) -> SpriteSet {
        let default_set = "test_char";
        return Self::get(sprite_library, default_set);
    }

    pub fn get(mut sprite_library: ResMut<SpriteLibrary>, name: &str) -> SpriteSet {
        return sprite_library.get(name);
    }

    pub fn create(name: &str, spr: HashMap<String, Sprite>) -> Self {
        SpriteSet {
            name: name.to_string(),
            atlases: spr,
        }
    }

    pub fn draw(&mut self, delta: Duration) {
        // Update and draw the sprite set using the provided delta time
        // This is a placeholder for the actual drawing logic
        // println!("Drawing sprite set: {} with delta: {:?}", self.name, delta);
    }
}
