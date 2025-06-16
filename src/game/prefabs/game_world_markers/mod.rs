use crate::game::prefabs::game_world::GameWorld;
use avian3d::prelude::{
    AngularInertia, CenterOfMass, Collider, ColliderConstructor, ColliderConstructorHierarchy,
    Mass, NoAutoAngularInertia, NoAutoCenterOfMass, NoAutoMass, RigidBody, VhacdParameters,
};
use bevy::ecs::query::QueryData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_auto_plugin::auto_plugin::*;
use smart_default::SmartDefault;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct EnemySpawnMarker;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct PlayerSpawnMarker;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct OutOfBoundsMarker;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct TemplePillar;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct TempleBase;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct TempleRoof;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct TempleLight;

#[auto_register_type]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
struct ColliderDisabled;

#[auto_register_type]
#[derive(Debug, Default, Copy, Clone, Reflect)]
pub enum Method {
    #[default]
    ConvexHull,
    ConvexDecomposition,
    ConvexDecompositionNoApprox,
    TriMesh,
}

#[auto_register_type]
#[derive(Component, Debug, SmartDefault, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct AutoColliderMesh {
    method: Method,
}

pub fn auto_collider_mesh_obs(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    auto_collider_mesh_q: Query<
        (Entity, Ref<AutoColliderMesh>, Option<&RigidBody>),
        Added<AutoColliderMesh>,
    >,
    added_mesh3d_q: Query<(Ref<Mesh3d>, Has<Collider>), Added<Mesh3d>>,
    children_q: Query<&Children>,
) {
    commands.entity(trigger.observer()).despawn();
    let entity = trigger.target();
    info!("Trigger<SceneInstanceReady> {entity}");
    for child in children_q.iter_descendants(entity) {
        let Ok((entity, auto_collider_mesh_ref, rigid_body_opt)) = auto_collider_mesh_q.get(child)
        else {
            continue;
        };
        if !auto_collider_mesh_ref.is_added() {
            continue;
        }
        let queue = vec![entity]
            .into_iter()
            .chain(children_q.iter_descendants(entity));
        for entity in queue {
            let Ok((mesh3d_ref, has_collider)) = added_mesh3d_q.get(entity) else {
                continue;
            };
            if !mesh3d_ref.is_added() || has_collider {
                continue;
            }
            info!("ConvexHullFromMesh {entity}");
            let mut entity_cmds = commands.entity(entity);
            let has_rigid_body = rigid_body_opt.is_some();
            let rigid_body = rigid_body_opt.copied().unwrap_or(RigidBody::Static);
            if matches!(rigid_body, RigidBody::Static) {
                // required for large meshes to prevent: assertion failed: self.is_normalized()
                //  avian3d::dynamics::rigid_body::mass_properties::update_mass_properties
                entity_cmds.insert((
                    NoAutoMass,
                    NoAutoAngularInertia,
                    NoAutoCenterOfMass,
                    Mass::ZERO,
                    AngularInertia::ZERO,
                    CenterOfMass::ZERO,
                ));
            }
            if !has_rigid_body {
                entity_cmds.insert(rigid_body);
            }
            match auto_collider_mesh_ref.method {
                Method::ConvexHull => {
                    entity_cmds.insert(ColliderConstructor::ConvexHullFromMesh);
                }
                Method::ConvexDecomposition => {
                    entity_cmds.insert(ColliderConstructor::ConvexDecompositionFromMesh);
                }
                Method::ConvexDecompositionNoApprox => {
                    entity_cmds.insert(ColliderConstructor::ConvexDecompositionFromMeshWithConfig(
                        VhacdParameters {
                            convex_hull_approximation: false,
                            ..Default::default()
                        },
                    ));
                }
                Method::TriMesh => {
                    entity_cmds.insert(ColliderConstructor::TrimeshFromMesh);
                }
            }
        }
    }
}

#[auto_register_type]
#[derive(Component, Debug, SmartDefault, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct AutoColliderMesh2 {
    method: Method,
}

pub fn auto_collider_mesh2(
    mut commands: Commands,
    auto_collider_mesh_q: Query<
        (Entity, &AutoColliderMesh2, Option<&RigidBody>),
        Added<AutoColliderMesh2>,
    >,
) {
    for (entity, auto_collider_mesh_ref, rigid_body_opt) in auto_collider_mesh_q.iter() {
        info!("AutoColliderMesh2 {entity}");
        let mut entity_cmds = commands.entity(entity);
        let rigid_body = rigid_body_opt.copied().unwrap_or(RigidBody::Static);
        if matches!(rigid_body, RigidBody::Static) {
            // required for large meshes to prevent: assertion failed: self.is_normalized()
            //  avian3d::dynamics::rigid_body::mass_properties::update_mass_properties
            entity_cmds.insert((
                NoAutoMass,
                NoAutoAngularInertia,
                NoAutoCenterOfMass,
                Mass::ZERO,
                AngularInertia::ZERO,
                CenterOfMass::ZERO,
            ));
        }
        entity_cmds.insert(rigid_body);
        entity_cmds.insert(ColliderConstructorHierarchy::new(
            match auto_collider_mesh_ref.method {
                Method::ConvexHull => ColliderConstructor::ConvexHullFromMesh,
                Method::ConvexDecomposition => ColliderConstructor::ConvexDecompositionFromMesh,
                Method::ConvexDecompositionNoApprox => {
                    ColliderConstructor::ConvexDecompositionFromMeshWithConfig(VhacdParameters {
                        convex_hull_approximation: false,
                        ..Default::default()
                    })
                }
                Method::TriMesh => ColliderConstructor::TrimeshFromMesh,
            },
        ));
    }
}

#[derive(QueryData)]
pub struct EntityWithGlobalTransformQueryData {
    pub entity: Entity,
    pub global_transform: Ref<'static, GlobalTransform>,
}

#[derive(QueryData)]
pub struct MarkerQueryData<T>
where
    T: Component,
{
    pub entity: Entity,
    pub transform: Ref<'static, Transform>,
    _marker: &'static T,
}

#[derive(SystemParam)]
pub struct SpawnHelper<'w, 's, Marker>
where
    Marker: Component + 'static + Send + Sync,
{
    pub commands: Commands<'w, 's>,
    pub world_ent_q: Single<'w, Entity, With<GameWorld>>,
    pub marker_q: Single<'w, MarkerQueryData<Marker>, With<Marker>>,
}

impl<'w, 's, Marker> SpawnHelper<'w, 's, Marker>
where
    Marker: Component,
{
    pub fn spawn_in(&mut self, bundle: impl Bundle, transform: Transform) -> Entity {
        let marker_mat = self.marker_q.transform.compute_matrix();
        let local_mat = transform.compute_matrix();
        let final_transform = Transform::from_matrix(marker_mat * local_mat);
        self.commands
            .spawn(bundle)
            .insert(final_transform)
            .insert(ChildOf(self.world_ent_q.entity()))
            .id()
    }

    pub fn target_entity(&self) -> Entity {
        self.marker_q.entity
    }
}

#[derive(SystemParam)]
pub struct GameWorldMarkerSystemParam<'w, 's> {
    pub player_spawn: SpawnHelper<'w, 's, PlayerSpawnMarker>,
    pub enemy_spawn: SpawnHelper<'w, 's, EnemySpawnMarker>,
}

impl GameWorldMarkerSystemParam<'_, '_> {
    pub fn spawn_in_player_spawn(&mut self, bundle: impl Bundle, transform: Transform) -> Entity {
        self.player_spawn.spawn_in(bundle, transform)
    }

    pub fn spawn_in_enemy_spawn(&mut self, bundle: impl Bundle, transform: Transform) -> Entity {
        self.enemy_spawn.spawn_in(bundle, transform)
    }
}

fn on_add_collider_disabled(trigger: Trigger<OnAdd, ColliderDisabled>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .remove::<ColliderDisabled>()
        .insert(avian3d::prelude::ColliderDisabled);
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    // app.add_observer(on_add_collider_disabled);
    app.add_observer(auto_collider_mesh_obs);
    // app.add_systems(Update, auto_collider_mesh2);
}
