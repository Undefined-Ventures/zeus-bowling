use crate::game::asset_tracking::LoadResource;
use avian3d::prelude::{Collider, Friction, Mass, Restitution};
use avian3d::prelude::{ColliderDisabled, RigidBody};
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Dynamic)]
pub struct BowlingBall;

pub const BOWLING_BALL_RADIUS: f32 = 0.108;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct BowlingBallAssets {
    #[dependency]
    pub bowling_ball: Handle<Scene>,
}

impl FromWorld for BowlingBallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bowling_ball: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/bowling/bowling_ball.glb")),
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<BowlingBallAssets>();
    app.add_observer(on_added);
}

#[derive(QueryData)]
struct BowlingBallQueryData {
    friction: Option<&'static Friction>,
    restitution: Option<&'static Restitution>,
    mass: Option<&'static Mass>,
}

fn on_added(
    trigger: Trigger<OnAdd, BowlingBall>,
    assets: Res<BowlingBallAssets>,
    bowling_ball_q: Query<BowlingBallQueryData, With<BowlingBall>>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let mut entity_cmds = commands.entity(entity);
    entity_cmds.insert((
        Collider::sphere(BOWLING_BALL_RADIUS),
        SceneRoot(assets.bowling_ball.clone()),
    ));
    let bb = bowling_ball_q.get(entity).expect("impossible");

    if bb.friction.is_none() {
        entity_cmds.insert(Friction::new(0.4));
    }
    if bb.restitution.is_none() {
        entity_cmds.insert(Restitution::new(0.001));
    }
    if bb.mass.is_none() {
        entity_cmds.insert(Mass(5.0));
    }
}
