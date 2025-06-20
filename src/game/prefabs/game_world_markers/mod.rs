use crate::game::prefabs::game_world::GameWorld;
use bevy::ecs::query::QueryData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

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
pub struct BowlingBallSpawnMarker;

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

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}
