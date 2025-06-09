use crate::game::asset_tracking::LoadResource;
use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Static)]
pub struct GameWorld;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameWorldAssets {
    #[dependency]
    pub scene: Handle<Scene>,
}

impl FromWorld for GameWorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            scene: assets.load(GltfAssetLabel::Scene(0).from_asset("models/world/world.glb")),
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<GameWorldAssets>();
    app.add_observer(on_added);
}

fn on_added(
    trigger: Trigger<OnAdd, GameWorld>,
    assets: Res<GameWorldAssets>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands
        .entity(entity)
        .insert(SceneRoot(assets.scene.clone()));
}
