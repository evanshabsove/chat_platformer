use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{ascii::spawn_ascii_sprite, ascii::AsciiSheet, mover::Mover, GRAV, TILE_SIZE};

pub struct PlayerPugin;

#[derive(Component, Inspectable)]
pub struct Player {}

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
        .insert(Player {})
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.4,
        })
        .insert(Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0))
        .insert(GravityScale(GRAV))
        .insert(Mover {
            speed: 100.0,
            is_jumping: false,
        })
        .id();
}
