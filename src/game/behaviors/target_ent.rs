use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(CollidingEntities)]
pub struct TargetEnt {
    pub target_ent: Entity,
}
