use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::GridCoords, LevelSelection};
use bevy_ecs_ldtk::LdtkEntity;
use crate::{text::DurationText, AppState};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Target;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct TargetEntity;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct TargetBundle {
    target_entity: TargetEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}

pub struct TargetPlugin;

pub struct TargetDestroyedEvent;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TargetDestroyedEvent>()
            .add_system(check_level_duration)
            .add_system(display_end_screen);
    }
}

fn check_level_duration(
    time: Res<Time>,
    mut text_query: Query<(&mut Text, &mut DurationText)>,
    target_query: Query<&mut Target>,
) {
    let mut targets = 0;
    for _target in target_query.iter() {
        targets += 1;
    }

    for (mut text, mut dration_text) in text_query.iter_mut() {
        if targets == 0 {
            // assume we have exactly one player that jumps with Spacebar
            text.sections[1].value = dration_text.time.elapsed_secs().to_string();
            dration_text.time.pause();
        } else {
            dration_text.time.unpause();
            // assume we have exactly one player that jumps with Spacebar
            text.sections[1].value = dration_text.time.elapsed_secs().to_string();
            dration_text.time.tick(time.delta());
        }
    } 
}

fn display_end_screen(
    mut app_state: ResMut<State<AppState>>,
    mut target_destroyed_event: EventReader<TargetDestroyedEvent>,
    target_query: Query<&mut Target>,
    mut level_selection: ResMut<LevelSelection>,
    keyboard: Res<Input<KeyCode>>,
) {
    for _event in target_destroyed_event.iter() {
        let mut targets = 0;
        for _target in target_query.iter() {
            targets += 1;
        }

        if targets == 0 {
            match app_state.current() {
                AppState::Level1 => {
                    app_state.set(AppState::Level2);
                    *level_selection = LevelSelection::Index(2);
                },
                AppState::Level2 => {
                    app_state.set(AppState::FinishScreen);
                },
                _ => {}
            }
        }
    }

    if keyboard.pressed(KeyCode::N) {
        let mut targets = 0;
        for _target in target_query.iter() {
            targets += 1;
        }

        if targets == 0 {
            match app_state.current() {
                AppState::Level1 => {
                    app_state.set(AppState::Level2);
                    *level_selection = LevelSelection::Index(2);
                },
                AppState::Level2 => {
                    app_state.set(AppState::FinishScreen);
                },
                _ => {}
            }
        }
    }
}
