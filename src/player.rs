use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{ascii::spawn_ascii_sprite, ascii::AsciiSheet, TILE_SIZE};

pub struct PlayerPugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

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
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 1.0 * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 1.0 * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 1.0 * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 1.0 * time.delta_seconds() * player.speed;
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 100.0 })
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
