use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};

mod ascii;
mod debug;
mod player;
mod tilemap;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPugin;
use tilemap::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 100.0;
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(AsciiPlugin)
        .add_plugin(PlayerPugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            left: -1000.0 * RESOLUTION,
            right: 1000.0 * RESOLUTION,
            top: 1000.0,
            bottom: -1000.0,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    };

    commands.spawn_bundle(camera);
}
