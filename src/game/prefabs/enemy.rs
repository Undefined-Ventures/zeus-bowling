use avian3d::prelude::{CollisionEventsEnabled, Gravity};

use crate::game::asset_tracking::LoadResource;
use crate::game::audio::sound_effect;
use crate::game::behaviors::MovementSpeed;
use crate::game::rng::global::GlobalRng;
use avian3d::prelude::{CenterOfMass, Collider, RigidBody};
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use rand::prelude::IndexedRandom;
use std::fmt::Debug;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    #[dependency]
    pub base_skele: Handle<Gltf>,
    // https://pixabay.com/sound-effects/bone-snap-295399/
    #[dependency]
    pub bone_snap_1: Handle<AudioSource>,
    // https://pixabay.com/sound-effects/bone-break-sound-269658/
    #[dependency]
    pub bone_snap_2: Handle<AudioSource>,
    pub bone_snap_sounds: Vec<Handle<AudioSource>>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let base_skele = assets.load("models/enemies/LowPolySkeletonRigged.glb");
        let bone_snap_1 = assets.load("audio/sound_effects/bone-snap-1.mp3");
        let bone_snap_2 = assets.load("audio/sound_effects/bone-snap-2.mp3");
        let bone_snap_sounds = vec![bone_snap_1.clone(), bone_snap_2.clone()];

        Self {
            base_skele,
            bone_snap_1,
            bone_snap_2,
            bone_snap_sounds,
        }
    }
}

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(CollisionEventsEnabled)]
pub enum Enemy {
    BaseSkele,
}

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Default, Copy, Clone, Reflect)]
pub struct PlayBoneSnap;

const DEFAULT_MOVE_SPEED: f32 = 30.0;
const DEFAULT_DESPAWN_AFTER_DEAD_SECS: f32 = 5.0;

impl Enemy {
    pub fn default_move_speed(&self) -> f32 {
        match self {
            Self::BaseSkele => DEFAULT_MOVE_SPEED,
        }
    }
    pub fn default_despawn_time(&self) -> f32 {
        match self {
            Self::BaseSkele => DEFAULT_DESPAWN_AFTER_DEAD_SECS,
        }
    }
}

fn on_enemy_added(
    trigger: Trigger<OnAdd, Enemy>,
    query: Query<&Enemy>,
    enemy_assets: Res<EnemyAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut commands: Commands,
    gravity: Res<Gravity>,
) {
    let enemy = query
        .get(trigger.target())
        .expect("No target entity for trigger");

    // Model handle
    let gltf_h = match *enemy {
        Enemy::BaseSkele => enemy_assets.base_skele.clone(),
    };
    let gltf = gltfs
        .get(&gltf_h)
        .unwrap_or_else(|| panic!("Missing gltf asset for {:?}", enemy));

    // MovementSpeed
    let movement_speed = MovementSpeed(enemy.default_move_speed());

    commands.entity(trigger.target()).insert((
        children![(
            SceneRoot(gltf.scenes[0].clone()),
            Transform::from_translation(Vec3::Y * -1.75),
        ),],
        // Parry colliders are centered around origin. Meshes have lowest
        // vertex at y=0.0. Spawning the collider allows us to adjust
        // its position to match the mesh.
        Collider::capsule(0.25, 3.0),
        CenterOfMass::new(0.0, -5.5, 0.0),
        RigidBody::Dynamic,
        movement_speed,
    ));
}

fn play_bone_snap(
    _trigger: Trigger<PlayBoneSnap>,
    mut global_rng: GlobalRng,
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
) {
    // TODO: spawn in world or state scope?
    commands.spawn(sound_effect(
        enemy_assets
            .bone_snap_sounds
            .choose(global_rng.rng())
            .unwrap()
            .clone(),
    ));
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>();
    app.add_observer(on_enemy_added);
    app.add_observer(play_bone_snap);
}
