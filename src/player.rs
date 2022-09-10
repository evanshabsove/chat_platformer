use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;

use crate::{ascii::spawn_ascii_sprite, ascii::AsciiSheet, tilemap::TileCollider, TILE_SIZE};

pub struct PlayerPugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_movement.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

fn camera_movement(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    // if keyboard.pressed(KeyCode::W) {
    //     y_delta += TILE_SIZE * time.delta_seconds() * player.speed;
    // }

    // if keyboard.pressed(KeyCode::S) {
    //     y_delta -= TILE_SIZE * time.delta_seconds() * player.speed;
    // }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= TILE_SIZE * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::D) {
        x_delta += TILE_SIZE * time.delta_seconds() * player.speed;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(1.0),
        );

        if collision.is_some() {
            return false;
        }
    }
    true
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(0.0, 0.0, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 5.0 })
        .id();

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands
        .entity(background)
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}
