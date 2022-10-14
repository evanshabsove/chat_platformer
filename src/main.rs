use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

mod ascii;
mod debug;
mod mover;
mod player;
mod stopwatch;
mod systems;
mod target;
mod text;
mod tilemap;
mod wall;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use mover::{Mover, MoverPlugin};
use player::PlayerPugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const GRAV: f32 = 3.0;
fn main() {
    let height: f32 = 900.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Chat Platformer".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LevelSelection::Index(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(spawn_map)
        .add_startup_system(spawn_camera)
        .add_plugin(AsciiPlugin)
        .add_plugin(PlayerPugin)
        .add_plugin(text::TextPlugin)
        .add_plugin(stopwatch::LevelDurationPlugin)
        // .add_plugin(DebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MoverPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(collision_events)
        .add_system(systems::spawn_wall_collision)
        .add_system(systems::spawn_target_collision)
        .register_ldtk_int_cell::<wall::WallBundle>(1)
        .register_ldtk_entity::<target::TargetBundle>("Target")
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            left: -100.0 * RESOLUTION,
            right: 100.0 * RESOLUTION,
            top: 100.0,
            bottom: -100.0,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    };

    commands.spawn_bundle(camera);
}

fn collision_events(
    mut mover_query: Query<(Entity, &mut Mover)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(_, entity, CollisionEventFlags::SENSOR) => {
                commands.entity(*entity).despawn_recursive();
            }
            CollisionEvent::Started(_, _, _) => {
                for (_entity, mut mover) in mover_query.iter_mut() {
                    set_jumping_false_if_touching_floor(&mut mover);
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn set_jumping_false_if_touching_floor(mover: &mut Mover) {
    mover.is_jumping = false;
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map_1.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
