use bevy::prelude::*;
use std::path::Path;
use crate::direction::{Direction8, CurrentDirection};
use crate::sprite_state::{SpriteState, CurrentSpriteState};

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(std::time::Duration::from_secs_f32(1.0 / fps as f32), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct ImageAtlasBundle {
    pub image_handle: Handle<Image>,
    pub path: String,
    pub direction: Direction8,
    pub state: SpriteState,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    // Load all combinations of direction + state
    for direction in Direction8::all() {
        for state in SpriteState::all() {
            let filename = format!("textures/{:?}_{:?}_3x7.png", direction, state).to_lowercase();
            let image_handle = asset_server.load(&filename);

            commands.spawn(ImageAtlasBundle {
                image_handle,
                path: filename,
                direction: *direction,
                state: *state,
            });
        }
    }
}

pub fn process_atlases(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    images: Res<Assets<Image>>,
    query: Query<(Entity, &ImageAtlasBundle)>,
) {
    for (entity, bundle) in &query {
        if let Some(image) = images.get(&bundle.image_handle) {
            if let Some((cols, rows)) = parse_grid_from_filename(&bundle.path) {
                let sprite_size = UVec2::new(image.width() / cols, image.height() / rows);
                let layout = TextureAtlasLayout::from_grid(sprite_size, cols, rows, None, None);
                let layout_handle = texture_atlas_layouts.add(layout);
                let total_frames = (cols * rows) as usize;
                let animation = AnimationConfig::new(0, total_frames - 1, 12);

                commands.entity(entity)
                    .insert((
                        Sprite {
                            image: bundle.image_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout_handle,
                                index: animation.first_sprite_index,
                            }),
                            ..default()
                        },
                        Visibility::Hidden, // start hidden until selected
                        Transform::from_scale(Vec3::splat(1.0)),
                        animation,
                        bundle.direction,
                        bundle.state,
                    ))
                    .remove::<ImageAtlasBundle>();
            }
        }
    }
}

pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn update_visibility(
    current_dir: Res<CurrentDirection>,
    current_state: Res<CurrentSpriteState>,
    mut query: Query<(&Direction8, &SpriteState, &mut Visibility)>,
) {
    for (sprite_dir, sprite_state, mut visibility) in &mut query {
        *visibility = if *sprite_dir == current_dir.direction && *sprite_state == current_state.state {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn parse_grid_from_filename(filename: &str) -> Option<(u32, u32)> {
    let stem = Path::new(filename).file_stem()?.to_str()?;
    if let Some((_, grid)) = stem.rsplit_once('_') {
        if let Some((cols, rows)) = grid.split_once('x') {
            return Some((cols.parse().ok()?, rows.parse().ok()?));
        }
    }
    None
}
