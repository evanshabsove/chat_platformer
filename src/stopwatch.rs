use bevy::prelude::*;

use crate::{target::Target, text::DurationText};

pub struct LevelDurationPlugin;

impl Plugin for LevelDurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_level_duration);
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
