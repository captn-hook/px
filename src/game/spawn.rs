// fn spawn_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // Example: spawn 3 characters
//     let positions = [Vec3::new(-100.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0)];
    
//     for pos in positions {
//         commands.spawn((
//             SpriteBundle {
//                 transform: Transform::from_translation(pos),
//                 texture: asset_server.load("sprites/character_still.png"),
//                 ..Default::default()
//             },
//             CurrentCharacterState::new(CharacterState::Still),
//             CurrentSpriteState::new(SpriteState::Still),
//             CurrentDirection::new(Direction8::South),
//         ));
//     }
// }
