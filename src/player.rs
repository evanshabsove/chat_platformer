use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{ascii::spawn_ascii_sprite, ascii::AsciiSheet};

pub struct PlayerPugin;

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_movement);
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
