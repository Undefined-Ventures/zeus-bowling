pub mod bowling_ball;
pub mod enemy;
pub mod game_world;
pub mod game_world_markers;
pub mod player;
pub mod spawner;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(enemy::plugin);
    app.add_plugins(spawner::plugin);
    app.add_plugins(bowling_ball::plugin);
    app.add_plugins(player::plugin);
    app.add_plugins(game_world::plugin);
    app.add_plugins(game_world_markers::plugin);
}
