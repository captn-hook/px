use crate::direction::Direction8;
use crate::game::character_state::CharacterState;
use crate::game::player_input::PlayerControl;
use crate::rendering::sprite_state::SpriteState;
use bevy::prelude::*;
use std::path::Path;

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            frame_timer: Timer::from_seconds(1.0 / fps as f32, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct ImageAtlasBundle {
    pub image_handle: Handle<Image>,
    pub path: String,
    pub direction: Direction8,
    pub state: SpriteState,
}

// ---------------------------------------------------------------------------
// State + Direction Driven Animation
// ---------------------------------------------------------------------------
pub fn execute_animations(
    time: Res<Time>,
    mut player_query: Query<(&Direction8, &mut SpriteState, &CharacterState), With<PlayerControl>>,
    mut query: Query<(&Direction8, &SpriteState, &mut AnimationConfig, &mut Sprite), Without<PlayerControl>>,
) {
    // Track if the currently playing animation has finished its loop
    let mut loop_completed = false;

    // read the player's authoritative components (expect exactly one Player)
    // copy values out so we don't hold a borrow while iterating other query
    let Ok((p_dir_ref, p_sprite_state_ref, p_char_state_ref)) = player_query.single() else { return };
    let player_dir = *p_dir_ref;
    let player_sprite_state = *p_sprite_state_ref;
    let player_char_state = *p_char_state_ref;

    for (dir, state, mut anim, mut sprite) in query.iter_mut() {
        if *dir != player_dir || *state != player_sprite_state {
            continue; // skip non-visible animations
        }

        anim.frame_timer.tick(time.delta());

        if anim.frame_timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < anim.last_sprite_index {
                    atlas.index += 1;
                } else {
                    // Last frame reached
                    atlas.index = anim.first_sprite_index;
                    loop_completed = true; // full loop completed
                }
            }

            anim.frame_timer.reset();
        }
    }

    // Only change sprite state if the **full animation loop has completed**
    let is_still_interruptible = player_sprite_state == SpriteState::Still;

    if loop_completed || is_still_interruptible {
        let next_state = match (player_char_state, player_sprite_state) {
            (CharacterState::Moving, SpriteState::Still | SpriteState::Stopping) => SpriteState::Starting,
            (CharacterState::Moving, SpriteState::Starting | SpriteState::Moving) => SpriteState::Moving,
            (CharacterState::Still, SpriteState::Starting | SpriteState::Moving) => SpriteState::Stopping,
            (CharacterState::Still, SpriteState::Stopping | SpriteState::Still) => SpriteState::Still,
        };
        if next_state != player_sprite_state {
            // acquire mutable access to the player's SpriteState now that we're done iterating
            if let Ok((_, mut player_sprite_state_mut, _)) = player_query.single_mut() {
                *player_sprite_state_mut = next_state;
            }

            // Reset the animation for the new state
            for (dir, state, mut anim, _) in query.iter_mut() {
                if *dir == player_dir && *state == next_state {
                    anim.frame_timer.reset();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Visibility Update
// ---------------------------------------------------------------------------

pub fn update_visibility(
    player_query: Query<(&Direction8, &SpriteState), With<PlayerControl>>,
    mut query: Query<(&Direction8, &SpriteState, &mut Visibility), Without<PlayerControl>>,
) {
    let Ok((player_dir, player_state)) = player_query.single() else { return };

    for (sprite_dir, sprite_state, mut visibility) in &mut query {
        *visibility =
            if *sprite_dir == *player_dir && *sprite_state == *player_state {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    for direction in Direction8::all() {
        for state in SpriteState::all() {
            let base = format!("textures/test_char/{:?}_{:?}", direction, state).to_lowercase();

            let filename = find_existing_texture(&base).unwrap_or_else(|| format!("{}.png", base));

            let (columns, rows) = parse_grid_from_filename(&filename).unwrap_or((1, 1));

            let image_handle = asset_server.load(&filename);

            commands.spawn(ImageAtlasBundle {
                image_handle,
                path: filename.clone(),
                direction: *direction,
                state: *state,
            });

            info!(
                "Loaded {:?} {:?} with grid {}x{}",
                direction, state, columns, rows
            );
        }
    }
}

/// Checks if either "base.png" or "base_NxM.png" exists.
/// Returns the first one that exists.
fn find_existing_texture(base: &str) -> Option<String> {
    let texture_dir = Path::new("assets");

    // check for "base.png"
    let plain_path = texture_dir.join(format!("{}.png", base));
    if plain_path.exists() {
        return Some(format!("{}.png", base));
    }

    // check for any "_NxM.png"
    if let Ok(entries) = std::fs::read_dir(texture_dir.join("textures/test_char")) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&base["textures/test_char/".len()..]) && name.ends_with(".png") {
                    return Some(format!("textures/test_char/{}", name));
                }
            }
        }
    }
    None
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

                commands
                    .entity(entity)
                    .insert((
                        Sprite {
                            image: bundle.image_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout_handle,
                                index: animation.first_sprite_index,
                            }),
                            ..default()
                        },
                        Visibility::Hidden, // start hidden; system will toggle visible ones
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

fn parse_grid_from_filename(filename: &str) -> Option<(u32, u32)> {
    let stem = Path::new(filename).file_stem()?.to_str()?;
    if let Some((_, grid)) = stem.rsplit_once('_') {
        if let Some((cols, rows)) = grid.split_once('x') {
            return Some((cols.parse().ok()?, rows.parse().ok()?));
        }
    }
    None
}
