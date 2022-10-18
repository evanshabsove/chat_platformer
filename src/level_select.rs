use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSelect {
  pub level: i32
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSelectEntity {
  pub level: i32
}

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct LevelSelectBundle {
    #[ldtk_entity]
    level_select_entity: LevelSelectEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl LdtkEntity for LevelSelectEntity {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> LevelSelectEntity {
        println!("LevelSelect added, here are some facts:");
        let mut level_value = 0;
        for field_instance in &entity_instance.field_instances {
          match field_instance.value {
                FieldValue::Int(Some(i)) => {
                  level_value = i;
                },
                _ => { println!("Not implemented"); }
            }
        }

        LevelSelectEntity {
          level: level_value
        }
    }
}

pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(AppState::OverWorld).with_system(remove_level_select));
    }
}

fn remove_level_select (
  mut level_select_query: Query<(Entity, &mut LevelSelect)>,
  mut commands: Commands,
) {
    for (level_select_entity, _level_select) in level_select_query.iter_mut() {
      commands.entity(level_select_entity).despawn_recursive();
    }
}