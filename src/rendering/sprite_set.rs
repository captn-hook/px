use bevy::prelude::*;
use std::collections::HashMap;
use crate::rendering::sprite_render::load_sprite;

/// A library containing all loaded sprite sets. Each set is keyed by a unique
/// name so different entities can refer to the same shared data by name.
#[derive(Resource)]
pub struct SpriteLibrary {
    /// Mapping from sprite-set name -> SpriteSet
    pub sets: HashMap<String, SpriteSet>,
}

impl Default for SpriteLibrary {
    fn default() -> Self {
        let mut sets = HashMap::new();
        let sprite_set = SpriteSet::create("test_char", load_sprite("test_char"));
        sets.insert("test_char".to_string(), sprite_set);
        Self {
            sets: sets
        }
    }
}

impl SpriteLibrary {
    pub fn get(&mut self, name: &str) -> SpriteSet {
        // if we have the sprite set in the library, return it
        if let Some(sprite_set) = self.sets.get(name) {
            return sprite_set.clone();
        } else {
            return self.add_sprite_set(name).clone();
        }
    }

    pub fn add_sprite_set(&mut self, name: &str) -> SpriteSet {
        // create a new sprite set and add it to the library
        let sprite_set = SpriteSet::create(name, load_sprite(name));
        self.sets.insert(name.to_string(), sprite_set);
        if let Some(sprite_set) = self.sets.get(name) {
            return sprite_set.clone();
        }
        panic!("Failed to add sprite set");
    }
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
}
