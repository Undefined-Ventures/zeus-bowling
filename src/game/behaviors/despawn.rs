use std::time::Duration;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{pause_controller::Pause, screens::Screen};

#[auto_register_type]
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Despawn {
    pub ttl: Duration,
}

impl Despawn {
    pub fn in_seconds(secs: f32) -> Self {
        Self {
            ttl: Duration::from_secs_f32(secs),
        }
    }
}

fn despawn(mut commands: Commands, time: Res<Time>, mut despawns: Query<(Entity, &mut Despawn)>) {
    for (entity, mut despawn) in despawns.iter_mut() {
        despawn.ttl = despawn.ttl.saturating_sub(time.delta());
        if despawn.ttl.is_zero() {
            if let Ok(mut ec) = commands.get_entity(entity) {
                ec.despawn();
            };
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn.run_if(in_state(Pause(false)).and(in_state(Screen::Gameplay))),
    );
}
