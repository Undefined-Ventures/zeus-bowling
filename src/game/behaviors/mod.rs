pub mod ball_counter;
pub mod camera;
pub mod despawn;
pub mod end_game;
pub mod spawn;
pub mod target_ent;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct MovementSpeed(pub f32);

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(ball_counter::plugin);
    app.add_plugins(camera::plugin);
    app.add_plugins(despawn::plugin);
    app.add_plugins(spawn::plugin);
}
