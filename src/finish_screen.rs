use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::AppState;

pub struct FinishScreenPlugin;

#[derive(Component, Inspectable)]
pub struct FinishScreen;

impl Plugin for FinishScreenPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::FinishScreen).with_system(setup_finish_screen))
      .add_system_set(SystemSet::on_exit(AppState::FinishScreen).with_system(remove_finish_screen));
  }
}

fn setup_finish_screen(mut commands: Commands,  asset_server: Res<AssetServer>) {
  commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::rgb(0.4, 0.4, 1.0).into(),
            ..default()
        })
        .with_children(|parent| {
          parent.spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "You did it!",
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
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        );
  })
  .insert(FinishScreen);
}

fn remove_finish_screen(mut commands: Commands, finish_screen_query: Query<Entity, With<FinishScreen>>) {
  let finish_screen_entity = finish_screen_query.single();

  commands.entity(finish_screen_entity).despawn_recursive()
}