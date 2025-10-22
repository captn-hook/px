use bevy::prelude::*;
use enum_iterator::Sequence;
use crate::rendering::sprite_set::Grid;
use crate::direction::Direction8;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub current: usize,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Sequence)]
pub enum SpriteState {
    Still,
    Starting,
    Moving,
    Stopping,
}

// to string
impl SpriteState {
    pub fn as_str(&self) -> &str {
        match self {
            SpriteState::Still => "still",
            SpriteState::Starting => "starting",
            SpriteState::Moving => "moving",
            SpriteState::Stopping => "stopping",
        }
    }

    pub fn from_str(s: &str) -> Option<SpriteState> {
        match s {
            "still" => Some(SpriteState::Still),
            "starting" => Some(SpriteState::Starting),
            "moving" => Some(SpriteState::Moving),
            "stopping" => Some(SpriteState::Stopping),
            _ => None,
        }
    }
}

#[derive(Bundle)]
pub struct SpriteBundle {
    pub direction: Direction8,
    pub state: SpriteState,
    pub sprite: Sprite,
    pub animation_timer: AnimationTimer,
    pub indices: AnimationIndices,
    pub visibility: Visibility,
    pub transform: Transform,
}

impl SpriteBundle {
    pub fn create(image: Handle<Image>, atlas: Handle<TextureAtlasLayout>, grid: &Grid) -> Self {
        
        return SpriteBundle {
            direction: grid.direction,
            state: grid.state,
            sprite: Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: atlas,
                    index: 0,
                },
            ),
            animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            indices: AnimationIndices {
                first: 0,
                last: (grid.sprites[0] * grid.sprites[1]) as usize - 1,
                current: 0,
            },
            visibility: Visibility::Hidden,
            transform: Transform::from_scale(Vec3::ONE / 2.0),
        };
    }
}
