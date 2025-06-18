use std::time::Duration;

use crate::game::asset_tracking::LoadResource;
use crate::game::audio::sound_effect;
use crate::game::behaviors::despawn::Despawn;
use crate::game::camera::CameraTarget;
use crate::game::prefabs::bowling_ball::BowlingBall;
use crate::game::prefabs::game_world_markers::BowlingBallSpawnMarker;
use crate::game::rng::global::GlobalRng;
use crate::game::scenes::LevelData;
use avian3d::prelude::{Collider, ExternalAngularImpulse, ExternalImpulse, Mass, RigidBody};
use bevy::ecs::event;
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

pub fn on_throw_ball_spawn_ball(
    mut throw_ball_event: EventReader<ThrowBallEvent>,
    mut commands: Commands,
    mut level_data: ResMut<LevelData>,
    player_q: Single<&Transform, With<Player>>,
    bowling_ball_spawn_q: Single<&GlobalTransform, With<BowlingBallSpawnMarker>>,
    player_assets: Res<PlayerAssets>,
) {
    let player_tf = player_q.into_inner();
    let ball_spawn_tf = bowling_ball_spawn_q.into_inner().compute_transform();
    for _event in throw_ball_event.read() {
        if level_data.balls_left > 0 {
            level_data.balls_left -= 1;
        }

        let power = 3.0;
        let player_dir = player_tf.back().as_vec3();
        commands.spawn((
            BowlingBall,
            CameraTarget,
            //ExternalAngularImpulse::new(player_dir_forward * (Vec3::X * 10.0 * power)),
            ExternalImpulse::new(player_dir * 1000. * power),
            Mass(20.0),
            Despawn {
                ttl: Duration::from_secs_f32(10.0),
            },
            Transform::from_scale(Vec3::splat(20.0)).with_translation(ball_spawn_tf.translation),
        ));

        // Sfx
        commands.spawn(sound_effect(
            player_assets
                .throw_sounds
                .choose(&mut rand::rng())
                .unwrap()
                .clone(),
        ));
    }
}

#[derive(Event)]
pub struct ThrowBallEvent;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>()
        .add_event::<ThrowBallEvent>()
        .add_systems(Update, debug)
        .add_systems(Update, on_throw_ball_spawn_ball)
        .add_observer(on_added);
}

fn debug(player_q: Single<(Entity, &Transform), With<Player>>, mut gizmos: Gizmos) {
    let trans = player_q.1.clone();
    gizmos.axes(trans, 10.0);
    gizmos.arrow(
        trans.translation,
        trans.translation + trans.forward() * 100.,
        Color::srgb(1.0, 0.2, 0.2),
    );
    gizmos.arrow(
        trans.translation,
        trans.translation + trans.back() * 100.,
        Color::srgb(1.0, 1.0, 0.2),
    );
}

fn on_added(trigger: Trigger<OnAdd, Player>, assets: Res<PlayerAssets>, mut commands: Commands) {
    let entity = trigger.target();
    commands.entity(entity).insert((
        SceneRoot(assets.scene.clone()),
        //Collider::capsule(3.0, 8.0)
    ));
}
