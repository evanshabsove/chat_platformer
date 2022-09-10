use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::TILE_SIZE;

const X_DIFF: f32 = 1.0;
const Y_DIFF: f32 = 5.0;
pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_map);
    }
}

fn build_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(
                        (x as f32 - X_DIFF) * TILE_SIZE,
                        -(y as f32 - Y_DIFF) * TILE_SIZE,
                        100.0,
                    ),
                );
                tiles.push(tile);

                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
            }
        }
    }

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::default(),
            visibility: Visibility { is_visible: true },
            global_transform: GlobalTransform::default(),
            ..default()
        })
        .push_children(&tiles);
}
