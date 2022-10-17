use crate::{player::Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalForce, Velocity};
pub struct MoverPlugin;

#[derive(Component)]
pub struct Mover {
    pub speed: f32,
    pub is_jumping: bool,
}

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_movement.label("movement"));
    }
}

fn player_movement(
    mut player_query: Query<(&mut Mover, &mut Velocity, &mut ExternalForce), With<Player>>,
    keyboard: ResMut<Input<KeyCode>>,
) {
    let (mut mover, mut velocity, mut force) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) && !mover.is_jumping {
        velocity.linvel.y = mover.speed;
        mover.is_jumping = true;
    }

    if keyboard.pressed(KeyCode::A) {
        let new_horizontal_force = calc_force_diff(velocity.linvel.x, -TARGET_TOP_SPEED);
        force.force.x = new_horizontal_force;
    } else if keyboard.pressed(KeyCode::D) {
        let new_horizontal_force = calc_force_diff(velocity.linvel.x, TARGET_TOP_SPEED);
        force.force.x = new_horizontal_force;
    } else {
        if velocity.linvel.x.abs() > 0.01 {
            let new_horizontal_force = -velocity.linvel.x;

            force.force.x = new_horizontal_force;
        }
    }
}

const TARGET_TOP_SPEED: f32 = 100.0;
/// clamped_input is a 0.0-1.0 value representing the user's
/// desired percentage of top speed to hold
///
/// `current_velocity` is the current horizontal velocity
fn calc_force_diff(current_velocity: f32, target_velocity: f32) -> f32 {
    let target_speed = target_velocity;
    let diff_to_make_up = target_speed - current_velocity;
    let new_force = diff_to_make_up * 2.0;
    new_force
}
