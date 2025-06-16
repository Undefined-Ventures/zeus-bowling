use super::LevelData;
use crate::game::behaviors::target_ent::TargetEnt;
use crate::game::pause_controller::Pause;
use crate::game::prefabs::enemy::Enemy;
use crate::game::prefabs::game_world::GameWorld;
use crate::game::prefabs::game_world_markers::{
    GameWorldMarkerSystemParam, auto_collider_mesh_obs,
};
use crate::game::prefabs::player::{Player, PlayerSystemParam};
use crate::game::screens::Screen;
use avian3d::prelude::{Friction, Mass};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_auto_plugin::auto_plugin::*;
use itertools::Itertools;
use smart_default::SmartDefault;
use std::time::Duration;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
pub struct LevelRoot;

pub fn spawn_level(mut commands: Commands) {
    info!("spawning world");
    commands
        .spawn((GameWorld, StateScoped(Screen::Gameplay)))
        .observe(auto_collider_mesh_obs)
        .observe(spawn_extras_on_instance_ready)
        .with_child((
            Name::new("Sun"),
            DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            CascadeShadowConfigBuilder {
                maximum_distance: 99999.9,
                ..Default::default()
            }
            .build(),
        ));
}

fn spawn_over_time(
    world: &World,
    mut commands: Commands,
    mut game_world_marker: GameWorldMarkerSystemParam,
    mut count_down: Local<Duration>,
    mut wave: Local<usize>,
    time: Res<Time>,
) {
    *count_down = count_down.saturating_sub(time.delta());
    if !count_down.is_zero() {
        return;
    }
    let time_between_waves = Duration::from_secs_f32(20.);
    *count_down = time_between_waves;
    if *wave == 0 {
        *wave = 1;
    }
    info!("spawning enemies");
    let formation_id = game_world_marker.spawn_in_enemy_spawn(
        (Name::new(format!("SkeleGroup({})", *wave)),),
        Transform::default(),
    );
    info!(
        "{:#?}",
        world
            .inspect_entity(formation_id)
            .map_or(vec![], |i| i.map(|info| info.name()).collect::<Vec<_>>())
    );
    info!("done spawning formation {}", formation_id);
    let layout_entries = generate_pin_layout(3.0, 0.5, 1, Facing::Toward);
    layout_entries
        .into_iter()
        .map(|entry| {
            let pin_id = commands
                .spawn((
                    ChildOf(formation_id),
                    Enemy::BaseSkele,
                    Mass(1.0),
                    Visibility::default(),
                    Friction::new(0.4),
                    TargetEnt {
                        target_ent: game_world_marker.player_spawn.target_entity(),
                    },
                    Transform::from_scale(Vec3::splat(4.0)).with_translation(entry.pos.extend(0.)),
                ))
                .id();
            let pin = Pin { entity: pin_id };
            (pin, entry)
        })
        .collect_vec();
}

fn spawn_extras_on_instance_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut game_world_marker: GameWorldMarkerSystemParam,
) {
    commands.entity(trigger.observer()).despawn();
    game_world_marker.spawn_in_player_spawn(Player, Transform::default());
}

#[derive(Debug, SmartDefault)]
struct PlayerData {
    #[default = 2.0]
    power: f32,
    #[default = 0.0]
    accuracy: f32,
    #[default = 30.0]
    turn_rate: f32,
}
fn demo_input(
    time: Res<Time>,
    mut commands: Commands,
    mut local: Local<(bool, PlayerData)>,
    mut player_system_param: PlayerSystemParam,
    mut level_data: ResMut<LevelData>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    let mut apply_transform = |transform: Transform| {
        commands
            .entity(player_system_param.entity())
            .insert(transform);
    };
    let (changed, cache) = &mut *local;
    let max_accuracy_offset: f32 = 30_f32.to_radians();
    if button_input.pressed(KeyCode::ArrowLeft) {
        cache.accuracy += 1_f32.to_radians();
        cache.accuracy = cache
            .accuracy
            .clamp(-max_accuracy_offset, max_accuracy_offset);
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowRight) {
        cache.accuracy -= 1_f32.to_radians();
        cache.accuracy = cache
            .accuracy
            .clamp(-max_accuracy_offset, max_accuracy_offset);
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowUp) {
        cache.power += 0.1;
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowDown) {
        cache.power -= 0.1;
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyW) {
        cache.turn_rate += 1.0;
        cache.turn_rate = cache.turn_rate.max(1.0);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyS) {
        cache.turn_rate -= 1.0;
        cache.turn_rate = cache.turn_rate.max(1.0);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyA) {
        let mut transform = player_system_param.player_transform();
        transform.rotate(Quat::from_rotation_y(
            1_f32.to_radians() * cache.turn_rate * time.delta_secs(),
        ));
        apply_transform(transform);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyD) {
        let mut transform = player_system_param.player_transform();
        transform.rotate(Quat::from_rotation_y(
            -1_f32.to_radians() * cache.turn_rate * time.delta_secs(),
        ));
        apply_transform(transform);
    }
    if button_input.just_pressed(KeyCode::Space) {
        if level_data.balls_left > 0 {
            player_system_param.spawn_bowling_ball(cache.power);
            level_data.balls_left -= 1;
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (demo_input, spawn_over_time)
            .run_if(in_state(Pause(false)).and(in_state(Screen::Gameplay))),
    );
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Away,
    Toward,
}

#[derive(Debug, Clone, Copy)]
struct Pin {
    entity: Entity,
}

#[derive(Debug, Clone, Copy)]
struct PinPosition {
    pos: Vec2,
    row: usize,
    col: usize,
}

pub fn generate_pin_layout(
    pin_width: f32,
    spacing: f32,
    rows: usize,
    facing: Facing,
) -> Vec<PinPosition> {
    let mut positions = Vec::new();
    for r in 0..rows {
        let num_in_row = (rows - r) as f32;
        let y = (r as f32) * (pin_width + spacing);
        // total width occupied by this row: N * pin_width + (N - 1) * spacing
        let row_width = num_in_row * pin_width + (num_in_row - 1.0) * spacing;

        // The first pin’s center x should be at:
        //   -row_width/2 + pin_width/2
        // so that the row is centered around x = 0.0
        let start_x = -row_width / 2.0 + pin_width / 2.0;

        for i in 0..(num_in_row as usize) {
            let x = start_x + (i as f32) * (pin_width + spacing);
            let y = match facing {
                Facing::Away => -y,
                Facing::Toward => y,
            };
            positions.push(PinPosition {
                pos: Vec2::new(x, y),
                row: r,
                col: i,
            });
        }
    }
    positions
}
