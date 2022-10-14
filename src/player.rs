use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{mover::Mover, GRAV, TILE_SIZE};

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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player/Character_004.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 30.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 900.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {})
        .insert(RigidBody::Dynamic)
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_STATIC)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0))
        .insert(GravityScale(GRAV))
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Mover {
            speed: 200.0,
            is_jumping: false,
        });
}
