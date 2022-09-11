use crate::player::Player;
use bevy::prelude::*;
pub struct MoverPlugin;

use crate::TILE_SIZE;

pub struct Mover {
    jump_impulse: f32,
    is_jumping: bool,
}

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement.label("movement"));
    }
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += TILE_SIZE * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::S) {
        y_delta -= TILE_SIZE * time.delta_seconds() * player.speed;
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= TILE_SIZE * time.delta_seconds() * player.speed;
    }

    if keyboard.pressed(KeyCode::D) {
        x_delta += TILE_SIZE * time.delta_seconds() * player.speed;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    // if wall_collision_check(target, &wall_query) {
    transform.translation = target;
    // }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    // if wall_collision_check(target, &wall_query) {
    transform.translation = target;
    // }
}
