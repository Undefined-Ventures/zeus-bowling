use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{pause_controller::Pause, scenes::LevelData, screens::Screen};

fn update_zeus_ball_count(time: Res<Time>, mut ld: ResMut<LevelData>) {
    ld.time_to_new_ball = ld.time_to_new_ball.saturating_sub(time.delta());
    if ld.time_to_new_ball.is_zero() {
        ld.balls_left += 1;
        ld.time_to_new_ball = ld.new_ball_rate;
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_zeus_ball_count.run_if(in_state(Pause(false)).and(in_state(Screen::Gameplay))),
    );
}
