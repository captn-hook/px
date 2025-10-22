use crate::direction::Direction8;
use crate::game::character_input::RandomInput;
use crate::game::character_state::CharacterState;
use crate::game::player_input::PlayerControl;
use crate::rendering::sprite_render::parse_grid_from_filename;
use crate::rendering::sprite_state::SpriteState;

use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub direction: Direction8,
    pub character_state: CharacterState,
    pub animation_timer: AnimationTimer,
    pub indices: AnimationIndices,
    pub sprite_state: SpriteState,
    pub sprite: Sprite,
    pub transform: Transform,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let filename = "textures/test_char/east_moving_1x11_500x500.png";
    if let Some(grid) = parse_grid_from_filename(filename) {
        // proceed with spawning
        let texture = asset_server.load(filename);
        let layout =
            TextureAtlasLayout::from_grid(grid.size, grid.sprites[0], grid.sprites[1], None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: (grid.sprites[0] * grid.sprites[1]) as usize - 1 };

        commands.spawn((
            CharacterBundle {
                direction: Direction8::South,
                character_state: CharacterState::Still,
                animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                indices: AnimationIndices { first: 0, last: 0 },
                sprite_state: SpriteState::Still,
                sprite: Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                ),
                transform: Transform::from_scale(Vec3::splat(3.0)),
            },
            PlayerControl::default(),
        ));
    } else {
        panic!("Failed to parse grid from filename: {}", filename);
    }
}

// pub fn spawn_character(mut commands: Commands, sprite_library: Res<SpriteLibrary>) {
//     commands.spawn((
//         CharacterBundle {
//             direction: Direction8::South,
//             sprite_state: SpriteState::Still,
//             animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
//             character_state: CharacterState::Still,
//             sprite_set: SpriteSet::default(sprite_library),
//             transform: Transform::from_translation(Vec3::X * 2.0),
//         },
//         RandomInput::default(),
//     ));
// }
