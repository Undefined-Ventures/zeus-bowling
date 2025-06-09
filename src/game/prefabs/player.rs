use std::time::Duration;

use crate::game::asset_tracking::LoadResource;
use crate::game::audio::sound_effect;
use crate::game::behaviors::despawn::Despawn;
use crate::game::camera::CameraTarget;
use crate::game::effects::lightning_ball::LightningBall;
use crate::game::prefabs::bowling_ball::BowlingBall;
use crate::game::prefabs::game_world::GameWorld;
use crate::game::prefabs::game_world_markers::{
    BowlingBallSpawnMarker, ComponentName, EntityWithGlobalTransformQueryData, SpawnHelper,
};
use crate::game::rng::global::GlobalRng;
use crate::game::scenes::LevelData;
use avian3d::prelude::{Collider, ExternalAngularImpulse, ExternalImpulse, Mass, RigidBody};
use bevy::audio::PlaybackMode;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use rand::seq::IndexedRandom;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Kinematic)]
pub struct Player;

impl ComponentName for Player {
    fn component_name() -> &'static str {
        "Player"
    }
}

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub scene: Handle<Scene>,
    // https://pixabay.com/sound-effects/whoosh-313320/
    #[dependency]
    pub throw_1: Handle<AudioSource>,
    pub throw_sounds: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let throw_1 = assets.load("audio/sound_effects/throw_1.mp3");
        let throw_sounds = vec![throw_1.clone()];
        Self {
            scene: assets.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("models/zeus/zeus_rigged_manual_bowling_ball.glb"),
            ),
            throw_1,
            throw_sounds,
        }
    }
}

#[derive(SystemParam)]
pub struct PlayerSystemParam<'w, 's> {
    commands: Commands<'w, 's>,
    pub player_transform: Single<'w, Ref<'static, Transform>, With<Player>>,
    player: SpawnHelper<'w, 's, GameWorld, Player>,
    player_assets: Res<'w, PlayerAssets>,
    pub bowling_ball_spawn: SpawnHelper<'w, 's, GameWorld, BowlingBallSpawnMarker>,
    rng: GlobalRng<'w, 's>,
}

impl PlayerSystemParam<'_, '_> {
    pub fn entity(&self) -> Entity {
        self.player.target_q.entity
    }
    pub fn get_player_rotation(&self) -> Quat {
        self.player.compute_target_local_transform(None).rotation
    }
    pub fn spawn_bowling_ball_spawn(
        &mut self,
        bundle: impl Bundle,
        transform: Option<Transform>,
    ) -> Entity {
        self.bowling_ball_spawn.spawn_in(bundle, transform)
    }
    pub fn spawn_bowling_ball(&mut self, power: f32, accuracy_offset_radians: f32) -> Entity {
        let player_rot = self.get_player_rotation();
        let accuracy_rot = player_rot * Quat::from_rotation_y(accuracy_offset_radians);
        let rng = self.rng.rng();
        self.commands.spawn(sound_effect(
            self.player_assets.throw_sounds.choose(rng).unwrap().clone(),
        ));
        let bowling_ball = self.spawn_bowling_ball_spawn(
            (
                BowlingBall,
                LightningBall,
                CameraTarget,
                ExternalAngularImpulse::new(accuracy_rot * (Vec3::X * 10.0 * power)),
                ExternalImpulse::new(accuracy_rot * (Vec3::Z * 1000.0 * power)),
                Mass(20.0),
                Despawn {
                    ttl: Duration::from_secs_f32(10.0),
                },
            ),
            Some(Transform::from_scale(Vec3::splat(20.0))),
        );
        bowling_ball
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.add_observer(on_added);
}

fn on_added(trigger: Trigger<OnAdd, Player>, assets: Res<PlayerAssets>, mut commands: Commands) {
    let entity = trigger.target();
    commands
        .entity(entity)
        .insert((SceneRoot(assets.scene.clone()), Collider::capsule(3.0, 8.0)));
}
