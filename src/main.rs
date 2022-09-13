use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod ascii;
mod debug;
mod mover;
mod player;
mod tilemap;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use mover::{Mover, MoverPlugin};
use player::PlayerPugin;
use tilemap::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 100.0;
pub const GRAV: f32 = 3.0;
fn main() {
    let height: f32 = 900.0;

    // App::new()
    //     .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(LdtkPlugin)
    //     .add_startup_system(setup)
    //     .insert_resource(LevelSelection::Index(0))
    //     .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
    //     .run();

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
        .add_plugin(TileMapPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MoverPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(jump_reset)
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn spawn_camera(mut commands: Commands) {
    // let camera = Camera2dBundle {
    //     projection: OrthographicProjection {
    //         left: -1000.0 * RESOLUTION,
    //         right: 1000.0 * RESOLUTION,
    //         top: 1000.0,
    //         bottom: -1000.0,
    //         scaling_mode: ScalingMode::None,
    //         ..default()
    //     },
    //     ..default()
    // };

    commands.spawn_bundle(Camera2dBundle::default());
}

fn jump_reset(
    mut query: Query<(Entity, &mut Mover)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for _collision_event in collision_events.iter() {
        for (_entity, mut mover) in query.iter_mut() {
            set_jumping_false_if_touching_floor(&mut mover);
        }
    }
}

fn set_jumping_false_if_touching_floor(mover: &mut Mover) {
    mover.is_jumping = false;
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(Camera2dBundle::default());

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
