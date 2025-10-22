use crate::direction::Direction8;
use crate::rendering::sprite_set::SpriteLibrary;
use crate::rendering::sprite_state::SpriteState;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use std::collections::HashMap;
use std::path::Path;
use enum_iterator::all;
use crate::load_sync::AssetBarrier;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn render_sprites(
    // windows: Query<&Window>, // for sizing (unused for now)
    mut sprite_library: ResMut<SpriteLibrary>,
    time: Res<Time>,
) {
    for (set_name, set) in sprite_library.sets.iter_mut() {
        // println!("Drawing sprite set: {}", set_name);
        set.draw(time.delta());
    }
}

pub fn load_sprite(name: &str, asset_server: Res<AssetServer>, assets: Res<Assets<Image>>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) -> HashMap<String, Sprite> {
    // Build a mapping for every Direction8 x SpriteState combination.

    let mut map: HashMap<String, Sprite> = HashMap::new();

    for direction in all::<Direction8>() {
        for state in all::<SpriteState>() {
            // get correct file name for this sprite
            let dir_state = format!("{}_{}", direction.as_str(), state.as_str());
            let file = find_existing_texture(name, &dir_state);
            if let Some(file) = file {
                println!("Found texture file: {}", file);
                let grid = parse_grid_from_filename(&file);
                println!("Grid for {}: {:?}", file, grid);
                if let Some(grid) = grid {

                    let (barrier, guard) = AssetBarrier::new();
                    let image_handle:Handle<Image> = asset_server.load_acquire(&file, guard.clone());
                    let future = barrier.wait_async();


                    AsyncComputeTaskPool::get()
                        .spawn(async move {
                            future.await;

                            let im = assets.get(image_handle.id());

                            if let Some(im) = im {
                                let im_size = im.size();
                                let layout = TextureAtlasLayout::from_grid(im_size, grid[0], grid[1], None, None);
                                let layout_handle = texture_atlas_layouts.add(layout);
                                let sprite = Sprite {
                                    image: image_handle,
                                    texture_atlas: Some(TextureAtlas {
                                        layout: layout_handle,
                                        index: 0,
                                    }),
                                    ..default()
                                };
                                map.insert(dir_state, sprite);
                            } else {
                                println!("Failed to load image for sprite: {}", dir_state);
                            }
                        })
                        .detach();
                }
            }
        }
    }

    return map;
}

// #[derive(Component, Clone)]
// pub struct AnimationConfig {
//     pub first_sprite_index: usize,
//     pub last_sprite_index: usize,
//     pub frame_timer: Timer,
// }

// impl AnimationConfig {
//     pub fn new(first: usize, last: usize, fps: u8) -> Self {
//         Self {
//             first_sprite_index: first,
//             last_sprite_index: last,
//             frame_timer: Timer::from_seconds(1.0 / fps as f32, TimerMode::Repeating),
//         }
//     }
// }

// ---------------------------------------------------------------------------
// State + Direction Driven Animation
// ---------------------------------------------------------------------------
// pub fn execute_animations(
//     time: Res<Time>,
//     mut player_query: Query<(&Direction8, &mut SpriteState, &CharacterState), With<PlayerControl>>,
//     mut query: Query<(&Direction8, &SpriteState, &mut AnimationConfig, &mut Sprite), Without<PlayerControl>>,
// ) {
//     // Track if the currently playing animation has finished its loop
//     let mut loop_completed = false;

//     // read the player's authoritative components (expect exactly one Player)
//     // copy values out so we don't hold a borrow while iterating other query
//     let Ok((p_dir_ref, p_sprite_state_ref, p_char_state_ref)) = player_query.single() else { return };
//     let player_dir = *p_dir_ref;
//     let player_sprite_state = *p_sprite_state_ref;
//     let player_char_state = *p_char_state_ref;

//     for (dir, state, mut anim, mut sprite) in query.iter_mut() {
//         if *dir != player_dir || *state != player_sprite_state {
//             continue; // skip non-visible animations
//         }

//         anim.frame_timer.tick(time.delta());

//         if anim.frame_timer.finished() {
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 if atlas.index < anim.last_sprite_index {
//                     atlas.index += 1;
//                 } else {
//                     // Last frame reached
//                     atlas.index = anim.first_sprite_index;
//                     loop_completed = true; // full loop completed
//                 }
//             }

//             anim.frame_timer.reset();
//         }
//     }

//     // Only change sprite state if the **full animation loop has completed**
//     let is_still_interruptible = player_sprite_state == SpriteState::Still;

//     if loop_completed || is_still_interruptible {
//         let next_state = match (player_char_state, player_sprite_state) {
//             (CharacterState::Moving, SpriteState::Still | SpriteState::Stopping) => SpriteState::Starting,
//             (CharacterState::Moving, SpriteState::Starting | SpriteState::Moving) => SpriteState::Moving,
//             (CharacterState::Still, SpriteState::Starting | SpriteState::Moving) => SpriteState::Stopping,
//             (CharacterState::Still, SpriteState::Stopping | SpriteState::Still) => SpriteState::Still,
//         };
//         if next_state != player_sprite_state {
//             // acquire mutable access to the player's SpriteState now that we're done iterating
//             if let Ok((_, mut player_sprite_state_mut, _)) = player_query.single_mut() {
//                 *player_sprite_state_mut = next_state;
//             }

//             // Reset the animation for the new state
//             for (dir, state, mut anim, _) in query.iter_mut() {
//                 if *dir == player_dir && *state == next_state {
//                     anim.frame_timer.reset();
//                 }
//             }
//         }
//     }
// }


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

fn parse_grid_from_filename(filename: &str) -> Option<UVec2> {
    let stem = Path::new(filename).file_stem()?.to_str()?;
    if let Some((_, grid)) = stem.rsplit_once('_') {
        if let Some((cols, rows)) = grid.split_once('x') {
            return Some(UVec2::new(
                cols.parse().ok()?,
                rows.parse().ok()?,
            ));
        }
    }
    return None;
}