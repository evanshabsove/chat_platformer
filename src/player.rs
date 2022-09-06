use bevy::prelude::*;

use crate::AsciiSheet;

pub struct PlayerPugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 100.0 * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(100.0));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .id();

    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgb(0.5, 0.5, 0.5);
    sprite.custom_size = Some(Vec2::splat(100.0));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}
