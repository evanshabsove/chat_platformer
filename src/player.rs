use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{mover::Mover, attacker::Attacker, GRAV, TILE_SIZE, MainCamera, AppState};

pub struct PlayerPugin;

#[derive(Component, Inspectable)]
pub struct Player {
    pub is_attacking: bool,
}

const RUN_INDEXES: [usize; 3] = [0, 1, 2];
const ATTACK_INDEXES: [usize; 3] = [3, 4, 5];

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(camera_movement)
        .add_system(animate_sprite_movement)
        .add_system(animate_sprite_attack)
        .add_system(spawn_hit_box)
        .add_system_set(SystemSet::on_enter(AppState::OverWorld).with_system(move_player_to_spawn))
        .add_system_set(SystemSet::on_enter(AppState::Level1).with_system(move_player_to_spawn));
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn camera_movement(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn move_player_to_spawn(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_query.single_mut();
    
    player_transform.translation.x = 100.0;
    player_transform.translation.y = 100.0;
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player/Character_004_Battler.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 900.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            is_attacking: false
        })
        .insert(RigidBody::Dynamic)
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_STATIC)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(GravityScale(GRAV))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(TILE_SIZE / 2.0, (TILE_SIZE / 2.0) + 4.0))
        .insert(Mover {
            speed: 200.0,
            is_jumping: false,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Attacker);
}

fn animate_sprite_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Player
    )>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (mut timer, mut sprite, player) in &mut query {
        if !player.is_attacking {            
            if keyboard.pressed(KeyCode::A) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.flip_x = false;
                    sprite.index = RUN_INDEXES[(sprite.index + 1) % RUN_INDEXES.len()];
                }
            } else if keyboard.pressed(KeyCode::D) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.flip_x = true;
                    sprite.index = RUN_INDEXES[(sprite.index + 1) % RUN_INDEXES.len()];
                }
            } else {
                sprite.index = 0;
            }
        }
    }
}

fn animate_sprite_attack(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Player
    )>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (mut timer, mut sprite, mut player) in &mut query {          
        if keyboard.pressed(KeyCode::Space) {
            player.is_attacking = true;

            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = ATTACK_INDEXES[(sprite.index + 1) % ATTACK_INDEXES.len()];
            }
        } else if player.is_attacking {
            timer.tick(time.delta());
            if timer.just_finished() {
                let index = ATTACK_INDEXES[(sprite.index + 1) % ATTACK_INDEXES.len()];
                sprite.index = index;

                if index == 5 {
                    player.is_attacking = false;
                }
            }
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Sword;

fn spawn_hit_box(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player)>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut sword_query: Query<(Entity, &mut Sword)>,
) {
    for (entity, mut player) in query.iter_mut() {
        if player.is_attacking {
            let mut player_transform = player_query.single_mut();
    
            commands.spawn()
                .insert(Sword)
                .insert(Collider::capsule(Vec2::new(TILE_SIZE, TILE_SIZE), Vec2::new(TILE_SIZE, TILE_SIZE), TILE_SIZE))
                .insert_bundle(
                    SpatialBundle {
                        transform: *player_transform,
                        ..Default::default()
                    }
                );
        } else {
            for (entity, mut sword) in sword_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}