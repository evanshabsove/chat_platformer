use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::GridCoords;
use bevy_ecs_ldtk::LdtkEntity;

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
