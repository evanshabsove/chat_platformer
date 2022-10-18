use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod attacker;
mod collisions;
mod debug;
mod level_select;
mod finish_screen;
mod mover;
mod player;
mod systems;
mod target;
mod text;
mod wall;

use attacker::Attacker;
use debug::DebugPlugin;
use finish_screen::FinishScreenPlugin;
use mover::MoverPlugin;
use player::PlayerPugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const GRAV: f32 = 3.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    OverWorld,
    Level1,
    Level2,
    FinishScreen
}
fn main() {
    let height: f32 = 900.0;

    App::new()
        .add_state(AppState::OverWorld)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Chat Platformer".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LevelSelection::Index(1))
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(spawn_map)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MoverPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FinishScreenPlugin)
        .add_system(systems::spawn_wall_collision)
        .add_system(systems::spawn_target_collision.label("spawn_targets"))
        .add_system(systems::spawn_level_select)
        .add_plugin(text::TextPlugin)
        .add_plugin(target::TargetPlugin)
        .add_plugin(collisions::CollisionsPlugin)
        .add_plugin(level_select::LevelSelectPlugin)
        .register_ldtk_int_cell::<wall::WallBundle>(1)
        .register_ldtk_entity::<target::TargetBundle>("Target")
        .register_ldtk_entity::<level_select::LevelSelectBundle>("Level_Select")
        .run();
}

#[derive(Component)]
pub struct MainCamera;

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

    commands.spawn_bundle(camera).insert(MainCamera);
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map_1.ldtk"),
        ..Default::default()
    });
}