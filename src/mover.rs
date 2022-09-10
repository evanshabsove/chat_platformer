use crate::player::Player;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
pub struct MoverPlugin;

use crate::{tilemap::TileCollider, TILE_SIZE};

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
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

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
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(1.0),
        );

        if collision.is_some() {
            return false;
        }
    }
    true
}

// fn player_jumps(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_query: Query<(&Mover, &mut RigidBodyVelocity), With<Player>>,
// ) {
//     let (mut player, mut transform) = player_query.single_mut();

//     for (mover, mut velocity) in players.iter_mut() {
//         if keyboard_input.pressed(KeyCode::W) {
//             velocity.linvel = Vec2::new(0., mover.jump_impulse).into();
//         }
//     }
// }
