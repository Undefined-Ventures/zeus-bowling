use std::time::Duration;

use super::enemy::Enemy;
use crate::game::screens::Screen;
use bevy::{color::palettes::css::RED, prelude::*};
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct Spawner {
    /// What enemy will get spawned.
    pub spawns: Enemy,
    /// How long it takes to spawn an enemy.
    pub spawn_duration: Duration,
    /// Countdown for next spawn.
    pub time_to_next_spawn: Duration,
    /// Number of entities this spawner will create.
    pub spawn_left: u32,
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_observer(on_spawner_added);
}

fn on_spawner_added(
    trigger: Trigger<OnAdd, Spawner>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<State<Screen>>,
) {
    if state.get() != &Screen::Gameplay {
        return;
    }
    commands.entity(trigger.target()).insert((
        Mesh3d(meshes.add(Cylinder::new(4.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(RED),
            perceptual_roughness: 1.0,
            ..Default::default()
        })),
    ));
}
