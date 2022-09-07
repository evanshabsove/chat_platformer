use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::TILE_SIZE;

pub struct TileMapPlugin;

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
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );
                tiles.push(tile);
            }
        }
    }

    // commands
    //     .spawn()
    //     .insert(Name::new("Map"))
    //     .insert(Transform::default())
    //     .insert(GlobalTransform::default())
    //     .insert(ComputedVisibility::default())
    //     .push_children(&tiles);
}
