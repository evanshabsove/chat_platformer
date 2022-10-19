use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_inspector_egui::Inspectable;

use crate::AppState;

pub struct FinishScreenPlugin;

#[derive(Component, Inspectable)]
pub struct FinishScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for FinishScreenPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::FinishScreen).with_system(setup_finish_screen))
      .add_system_set(SystemSet::on_exit(AppState::FinishScreen).with_system(remove_finish_screen))
      .add_system(button_system);
  }
}

fn setup_finish_screen(mut commands: Commands,  asset_server: Res<AssetServer>) {
  commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        );

        parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
  })
  .insert(FinishScreen);
}

fn remove_finish_screen(mut commands: Commands, finish_screen_query: Query<Entity, With<FinishScreen>>) {
  let finish_screen_entity = finish_screen_query.single();

  commands.entity(finish_screen_entity).despawn_recursive()
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>,
    mut level_selection: ResMut<LevelSelection>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
          Interaction::Clicked => {
              text.sections[0].value = "Restart".to_string();
              *color = PRESSED_BUTTON.into();
              app_state.set(AppState::OverWorld);
              *level_selection = LevelSelection::Index(1);
          }
          Interaction::None => {
              text.sections[0].value = "Restart".to_string();
              *color = NORMAL_BUTTON.into();
          }
          Interaction::Hovered => {},
      }
  }
}