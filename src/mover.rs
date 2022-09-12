use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{ActiveEvents, ExternalForce, Velocity},
    rapier::prelude::CollisionEvent,
};
pub struct MoverPlugin;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct Mover {
    pub speed: f32,
    pub is_jumping: bool,
}

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement.label("movement"));
    }
}

fn player_movement(
    mut player_query: Query<(&mut Mover, &mut Velocity, &mut ExternalForce), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut mover, mut velocity, mut force) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) && !mover.is_jumping {
        velocity.linvel.y = mover.speed;
        mover.is_jumping = true;
        // y_delta += TILE_SIZE * time.delta_seconds() * player.speed;
    }

    // if keyboard.pressed(KeyCode::S) {
    //     y_delta -= TILE_SIZE * time.delta_seconds() * player.speed;
    // }

    // let mut x_delta = 0.0;
    // if keyboard.pressed(KeyCode::A) {
    //     velocity.linvel.x = -(mover.speed);
    // }

    // if keyboard.pressed(KeyCode::D) {
    //     velocity.linvel.x = (mover.speed);
    // }

    if keyboard.pressed(KeyCode::A) {
        force.force.x = -(100.0);
    }

    if keyboard.pressed(KeyCode::D) {
        force.force.x = 100.0;
    }

    // let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    // // if wall_collision_check(target, &wall_query) {
    // transform.translation = target;
    // // }

    // let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    // // if wall_collision_check(target, &wall_query) {
    // transform.translation = target;
    // }
}
