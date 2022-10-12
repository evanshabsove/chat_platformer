use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::target::Target;
pub struct TextPlugin;

#[derive(Component, Inspectable)]
pub struct ScoreText;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_text).add_system(alter_text);
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_sections([
                TextSection::new(
                    "3",
                    TextStyle {
                        font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    " Targets Left",
                    TextStyle {
                        font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
            ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ScoreText);
}

fn alter_text(
    mut commands: Commands,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    mut target_query: Query<(&mut Target)>,
) {
    let mut targets = 0;
    for target in target_query.iter() {
        targets += 1;
    }
    for mut text in &mut text_query {
        text.sections[0].value = format!("{targets}");
    }
}
