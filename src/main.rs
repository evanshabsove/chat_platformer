use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

mod ascii;
mod attacker;
mod debug;
mod level_select;
mod finish_screen;
mod mover;
mod player;
mod stopwatch;
mod systems;
mod target;
mod text;
mod tilemap;
mod wall;

use attacker::Attacker;
use ascii::AsciiPlugin;
use debug::DebugPlugin;
use finish_screen::FinishScreenPlugin;
use level_select::LevelSelect;
use mover::{Mover, MoverPlugin};
use player::{Player, PlayerPugin};
use target::Target;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const GRAV: f32 = 3.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    OverWorld,
    Level1
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
        .add_plugin(AsciiPlugin)
        .add_plugin(PlayerPugin)
        .add_plugin(text::TextPlugin)
        .add_plugin(stopwatch::LevelDurationPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MoverPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FinishScreenPlugin)
        .add_system(collision_events)
        .add_system(systems::spawn_wall_collision)
        .add_system(systems::spawn_target_collision)
        .add_system(systems::spawn_level_select)
        .register_ldtk_int_cell::<wall::WallBundle>(1)
        .register_ldtk_entity::<target::TargetBundle>("Target")
        .register_ldtk_entity::<level_select::LevelSelectBundle>("Level_Select")
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        // projection: OrthographicProjection {
        //     left: -100.0 * RESOLUTION,
        //     right: 100.0 * RESOLUTION,
        //     top: 100.0,
        //     bottom: -100.0,
        //     scaling_mode: ScalingMode::None,
        //     ..default()
        // },
        ..default()
    };

    commands.spawn_bundle(camera).insert(MainCamera);
}

fn collision_events(
    mut mover_query: Query<(Entity, &mut Mover)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut target_query: Query<(Entity, &mut Target)>,
    mut level_select_query: Query<(Entity, &mut LevelSelect)>,
    mut level_selection: ResMut<LevelSelection>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(_, entity, CollisionEventFlags::SENSOR) => {
                for (target_entity, _target) in target_query.iter_mut() {
                    if entity.id() == target_entity.id() {
                        commands.entity(*entity).despawn_recursive();
                    }
                }

                for (level_select_entity, level_select) in level_select_query.iter_mut() {
                    if entity.id() == level_select_entity.id() {
                        *level_selection = LevelSelection::Index(level_select.level as usize);
                        let mut player_transform = player_query.single_mut();
                        player_transform.translation.x = 100.0;
                        player_transform.translation.y = 100.0;
                    }
                }
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
