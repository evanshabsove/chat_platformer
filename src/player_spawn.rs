use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::GridCoords};
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerSpawn;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerSpawnEntity;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    player_spawn_entity: PlayerSpawnEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}