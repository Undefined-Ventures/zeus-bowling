//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use avian3d::prelude::{Collider, ColliderConstructor, ColliderConstructorHierarchy};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::prefabs::game_world::GameWorld;
use crate::game::{asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Default, Copy, Clone, Reflect)]
struct PreloadWorld;

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Debug, Default, Copy, Clone, Reflect)]
struct ColliderRefCount {
    existing_colliders: usize,
    new_colliders: usize,
    new_collider_constructors: usize,
    existing_collider_constructors: usize,
    removed_collider_constructors: usize,
    game_worlds: usize,
}

impl ColliderRefCount {
    fn is_all_done(&self) -> bool {
        info!("{self:?}",);
        self.game_worlds == 1
            && self.new_collider_constructors > 0
            && self.existing_collider_constructors + self.new_collider_constructors
                == self.new_colliders
    }
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("PreLoadColliders Screen"),
        StateScoped(Screen::PreloadColliders),
        children![widget::label("Pre Loading Colliders...")],
    ));
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[derive(SystemParam)]
struct PreLoadSystemParam<'w, 's> {
    counts: ResMut<'w, ColliderRefCount>,
    game_worlds: Query<'w, 's, Entity, With<GameWorld>>,
    colliders: Query<'w, 's, Entity, With<Collider>>,
    added_colliders: Query<'w, 's, Entity, Added<Collider>>,
    collider_hierarchies: Query<'w, 's, Entity, With<ColliderConstructor>>,
    added_collider_hierarchies: Query<'w, 's, Entity, Added<ColliderConstructor>>,
    removed_collider_hierarchies: RemovedComponents<'w, 's, ColliderConstructor>,
}

impl PreLoadSystemParam<'_, '_> {
    fn init_counts(&mut self) {
        assert_eq!(self.game_worlds.iter().count(), 0);
        self.counts.existing_colliders = self.colliders.iter().count();
        self.counts.existing_collider_constructors = self.collider_hierarchies.iter().count();
    }
    fn update_counts(&mut self) {
        self.counts.game_worlds = self.game_worlds.iter().count();
        self.counts.new_colliders += self.added_colliders.iter().count();
        self.counts.new_collider_constructors += self.added_collider_hierarchies.iter().count();
        self.counts.removed_collider_constructors +=
            self.removed_collider_hierarchies.read().count();
    }
}

fn preload_world_a(mut commands: Commands) {
    info!("Preloading world a");
    commands.send_event(PreloadWorld);
}

fn preload_world_b(
    mut event_count: Local<usize>,
    mut commands: Commands,
    mut pre_load_sp: PreLoadSystemParam,
    mut events: EventReader<PreloadWorld>,
) {
    for _ in events.read() {
        *event_count += 1;
        info!("Preloading world event {}", *event_count);
        match *event_count {
            1 => {
                pre_load_sp.init_counts();
                info!("Preloading world b init counts {:?}", pre_load_sp.counts);
                commands.send_event(PreloadWorld);
            }
            2 => {
                info!("Preloading world b spawn game world");
                commands.spawn((GameWorld, StateScoped(Screen::PreloadColliders)));
            }
            _ => return,
        }
    }
}

fn ch_added(
    trigger: Trigger<OnAdd, (Collider, ColliderConstructorHierarchy)>,
    state: Res<State<Screen>>,
    mut commands: Commands,
    mut pre_load_sp: PreLoadSystemParam,
) {
    if state.get() != &Screen::PreloadColliders {
        commands.entity(trigger.observer()).despawn();
    }
    pre_load_sp.update_counts();

    info!("Preload update counts {:?}", pre_load_sp.counts);
}

fn add_obs(mut commands: Commands) {
    commands.add_observer(ch_added);
}
fn all_colliders_loaded(counts: Res<ColliderRefCount>, state: Res<State<Screen>>) -> bool {
    if state.get() != &Screen::PreloadColliders {
        return true;
    }
    counts.is_all_done()
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::PreloadColliders),
        (add_obs, spawn_loading_screen, preload_world_a).chain(),
    );

    app.add_systems(
        Update,
        (
            enter_gameplay_screen.run_if(all_colliders_loaded),
            preload_world_b,
        )
            .chain()
            .run_if(in_state(Screen::PreloadColliders)),
    );
}
