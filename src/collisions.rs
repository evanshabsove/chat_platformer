use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use bevy_ecs_ldtk::prelude::*;

use crate::{mover::Mover, target::{Target, TargetDestroyedEvent}, level_select::LevelSelect, AppState, player::Sword};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_events);
    }
}

fn collision_events(
    mut mover_query: Query<(Entity, &mut Mover)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut target_query: Query<(Entity, &mut Target)>,
    mut level_select_query: Query<(Entity, &mut LevelSelect)>,
    mut level_selection: ResMut<LevelSelection>,
    mut app_state: ResMut<State<AppState>>,
    mut target_destroyed_event: EventWriter<TargetDestroyedEvent>,
    mut sword_query: Query<(Entity, &mut Sword)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity, entity_2, CollisionEventFlags::SENSOR) => {

                for (sword_entity, _sword) in sword_query.iter_mut() {
                    if entity.id() == sword_entity.id() || entity_2.id() == sword_entity.id() {
                        for (target_entity, _target) in target_query.iter_mut() {
                            if entity_2.id() == target_entity.id() || entity.id() == target_entity.id() {
                                commands.entity(target_entity).despawn_recursive();
                                target_destroyed_event.send(TargetDestroyedEvent);
                            }
                        }
                    }
                } 

                for (level_select_entity, level_select) in level_select_query.iter_mut() {
                    if entity_2.id() == level_select_entity.id() {
                        *level_selection = LevelSelection::Index(level_select.level as usize);
                        app_state.set(AppState::Level1);
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

