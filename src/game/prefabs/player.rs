use std::f32::consts::PI;
use std::time::Duration;

use crate::game::asset_tracking::LoadResource;
use crate::game::audio::sound_effect;
use crate::game::behaviors::despawn::Despawn;
use crate::game::camera::CameraTarget;
use crate::game::prefabs::bowling_ball::BowlingBall;
use crate::game::rng::global::GlobalRng;
use avian3d::prelude::{Collider, ExternalAngularImpulse, ExternalImpulse, Mass, RigidBody};
use bevy::ecs::query::QueryData;
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

#[derive(QueryData)]
pub struct PlayerQD {
    pub entity: Entity,
    pub transform: &'static Transform,
}

#[derive(SystemParam)]
pub struct PlayerSystemParam<'w, 's> {
    commands: Commands<'w, 's>,
    player_q: Single<'w, PlayerQD, With<Player>>,
    player_assets: Res<'w, PlayerAssets>,
    rng: GlobalRng<'w, 's>,
    gizmos: Gizmos<'w, 's>,
}

impl PlayerSystemParam<'_, '_> {
    pub fn entity(&self) -> Entity {
        self.player_q.entity
    }

    pub fn player_transform(&self) -> Transform {
        self.player_q.transform.clone()
    }

    pub fn debug_aim(&mut self) {
        let trans = self.player_transform();
        self.gizmos.axes(trans, 10.0);
        self.gizmos.arrow(
            trans.translation,
            trans.translation + trans.forward() * 100.,
            Color::srgb(1.0, 0.2, 0.2),
        );
        self.gizmos.arrow(
            trans.translation,
            trans.translation + trans.back() * 100.,
            Color::srgb(1.0, 1.0, 0.2),
        );
    }

    pub fn spawn_bowling_ball(&mut self, power: f32) -> Entity {
        let player_dir = self.player_transform().back().as_vec3();
        let bowling_ball = self
            .commands
            .spawn((
                BowlingBall,
                CameraTarget,
                //ExternalAngularImpulse::new(player_dir_forward * (Vec3::X * 10.0 * power)),
                ExternalImpulse::new(player_dir * 1000. * power),
                Mass(20.0),
                Despawn {
                    ttl: Duration::from_secs_f32(10.0),
                },
                Transform::from_scale(Vec3::splat(20.0))
                    .with_translation(self.player_transform().translation),
            ))
            .id();

        // Sfx
        let rng = self.rng.rng();
        self.commands.spawn(sound_effect(
            self.player_assets.throw_sounds.choose(rng).unwrap().clone(),
        ));

        bowling_ball
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>()
        .add_systems(Update, debug)
        .add_observer(on_added);
}

fn debug(mut player_system_param: PlayerSystemParam) {
    player_system_param.debug_aim();
}

fn on_added(trigger: Trigger<OnAdd, Player>, assets: Res<PlayerAssets>, mut commands: Commands) {
    let entity = trigger.target();
    commands.entity(entity).insert((
        SceneRoot(assets.scene.clone()),
        //Collider::capsule(3.0, 8.0)
    ));
}
