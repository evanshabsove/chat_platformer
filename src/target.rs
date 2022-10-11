use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Target;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TargetBundle {
    target: Target,
}
