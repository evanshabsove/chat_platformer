use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

use crate::{TILE_SIZE, AppState};

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Instruction {
  pub instructions: String
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct InstructionEntity {
  pub instructions: String
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct InstructionBox;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct InstructionBundle {
    #[ldtk_entity]
    instruction_entity: InstructionEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl LdtkEntity for InstructionEntity {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> InstructionEntity {
        let mut string = String::from("");
        for field_instance in &entity_instance.field_instances {
          match &field_instance.value {
                FieldValue::String(Some(i)) => {
                  string = i.to_owned();
                },
                _ => { println!("Not implemented"); }
            }
        }

        InstructionEntity {
          instructions: string
        }
    }
}

pub struct InstructionPlugin;

impl Plugin for InstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_instructions)
           .add_system(spawn_instruction_collision_tiles)
           .add_system_set(SystemSet::on_exit(AppState::OverWorld).with_system(remove_instructions));
    }
}

fn spawn_instruction_collision_tiles (
    mut commands: Commands,
    instruction_query: Query<(&GridCoords, &InstructionEntity), Added<InstructionEntity>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    instruction_query.for_each(|(&grid_coords, instruction_entity)| {
        let texture_handle = asset_server.load("mystic_woods_free_v0.2/sprites/objects/objects.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 14, 9);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        grid_coords.x as f32 * TILE_SIZE,
                        grid_coords.y as f32 * TILE_SIZE,
                        100.0,
                    ),
                    ..Default::default()
                },
                ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0))
            .insert(Sensor)
            .insert(GlobalTransform::default())
            .insert(Instruction {
                instructions: instruction_entity.clone().instructions
            });
    });
}

fn show_instructions (
  mut instruction_query: Query<(Entity, &Transform, &mut Instruction)>,
  mut instruction_box_query: Query<(Entity, &InstructionBox)>,
  mut collision_events: EventReader<CollisionEvent>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  for collision_event in collision_events.iter() {
    match collision_event { 
          CollisionEvent::Started(entity, entity_2, CollisionEventFlags::SENSOR) => { 
                for (instruction_entity, transform, instruction) in instruction_query.iter_mut() {
                    if entity_2.id() == instruction_entity.id() {
                      println!("Ran over instructions");
                      commands
                          .spawn_bundle(ColorMesh2dBundle {
                              mesh: meshes
                                  .add(Mesh::from(Quad::new(Vec2::new(TILE_SIZE * 3.0, TILE_SIZE * 3.0))))
                                  .into(),
                              transform: Transform::from_translation(Vec3 {
                                  x: transform.translation.x,
                                  y: transform.translation.y + (TILE_SIZE * 4.0),
                                  z: transform.translation.z,
                              }),
                              visibility: Visibility { is_visible: true },
                              ..Default::default()
                          })
                          .with_children(|parent| {
                              let style = bevy::text::TextStyle {
                                  font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                                  font_size: 15.0,
                                  color: Color::WHITE,
                              };
                              parent.spawn_bundle(Text2dBundle {
                                  text: Text::from_section(&instruction.instructions, style)
                                      .with_alignment(TextAlignment::TOP_CENTER),
                                  transform: Transform {
                                      translation: Vec3 {
                                          x: 0.0,
                                          y: 10.0,
                                          z: 10.0,
                                      },
                                      scale: Vec3::splat(0.4),
                                      ..Default::default()
                                  },
                                  ..Default::default()
                              });
                          }).insert(InstructionBox);
                    }
                }
            }
            CollisionEvent::Started(_, _, _) => {
            }
            CollisionEvent::Stopped(entity, entity_2, CollisionEventFlags::SENSOR) => {
              for (instruction_box_entity, instruction_box) in instruction_box_query.iter_mut() {
                commands.entity(instruction_box_entity).despawn_recursive()
              }
            }
            CollisionEvent::Stopped(_,_,_) => {

            }
    }
  }
}

fn remove_instructions (
  mut instruction_query: Query<(Entity, &mut Instruction)>,
  mut instruction_box_query: Query<(Entity, &mut InstructionBox)>,
  mut commands: Commands,
) {
    for (instruction_entity, _instruction) in instruction_query.iter_mut() {
      commands.entity(instruction_entity).despawn_recursive();
    }

    for (instruction_box_entity, _instruction) in instruction_box_query.iter_mut() {
      commands.entity(instruction_box_entity).despawn_recursive();
    }
}