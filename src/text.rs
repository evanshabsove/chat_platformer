use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
pub struct TextPlugin;

#[derive(Component, Inspectable)]
pub struct Text {}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_text);
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ColorText);
}
