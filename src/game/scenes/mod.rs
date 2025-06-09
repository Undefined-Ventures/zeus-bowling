use std::time::Duration;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use smart_default::SmartDefault;

pub mod game;
pub mod ui;

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, SmartDefault, Debug, Clone, Reflect)]
pub struct LevelData {
    #[default(5)]
    pub temple_health: usize,
    pub kill_count: usize,
    #[default(2)]
    pub balls_left: usize,
    #[default(Duration::from_secs_f32(8.0))]
    pub new_ball_rate: Duration,
    #[default(Duration::from_secs_f32(8.0))]
    pub time_to_new_ball: Duration,
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(game::plugin);
    app.add_plugins(ui::plugin);
}
