use bevy::prelude::*;
use std::collections::HashMap;

/// A library containing all loaded sprite sets. Each set is keyed by a unique
/// name so different entities can refer to the same shared data by name.
#[derive(Resource, Default)]
pub struct SpriteLibrary {
    /// Mapping from sprite-set name -> SpriteSet
    pub sets: HashMap<String, SpriteSet>,
}

/// A reusable collection of texture atlases for a character or object.
///
/// Internally this stores a map from a string key (for example
/// "north_idle" or "south_walk") to a `Handle<TextureAtlas>`. Handles are
/// cheap to clone so `SpriteSet` can be shared across many entities. The
/// library owns the map and systems create components that reference the
/// handles they need.
pub struct SpriteSet {
    /// Human-friendly name for this set (used as the key in `SpriteLibrary`).
    pub name: String,

    /// Mapping from a small key (direction+state) to a TextureAtlas handle.
    /// Use a compact textual key to avoid tight coupling with enums in other
    /// modules; callers can use `format!("{}_{}", dir, state)` or similar.
    pub atlases: HashMap<String, Handle<TextureAtlasLayout>>,

    /// Optional fallback atlas to use when a specific key is missing.
    pub default: Option<Handle<TextureAtlasLayout>>,
}

impl SpriteSet {
    /// Create a new, empty sprite set with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            atlases: HashMap::new(),
            default: None,
        }
    }

    /// Insert an atlas handle for `key`. `key` should be a small string like
    /// "north_idle". Handles are cloned into the set.
    pub fn insert(&mut self, key: impl Into<String>, handle: Handle<TextureAtlasLayout>) {
        self.atlases.insert(key.into(), handle);
    }

    /// Set the default atlas used as a fallback when a key is missing.
    pub fn set_default(&mut self, handle: Handle<TextureAtlasLayout>) {
        self.default = Some(handle);
    }

    /// Get a handle for `key`. Returns the specific atlas if present, or the
    /// fallback default atlas if set, otherwise `None`.
    pub fn get(&self, key: &str) -> Option<Handle<TextureAtlasLayout>> {
        self.atlases
            .get(key)
            .cloned()
            .or_else(|| self.default.clone())
    }

    /// Returns true when this set contains a handle for `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.atlases.contains_key(key)
    }
}
