use crate::game::{scenes::game::spawn_level, screens::Screen, theme::prelude::*};
use bevy::{prelude::*, scene::SceneInstance};
use bevy_auto_plugin::auto_plugin::*;

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Level"),
        BackgroundColor(Color::BLACK),
        StateScoped(Screen::LoadLevel),
        children![widget::label("Loading Level...")],
    ));
}

fn monitor_load_completion(
    mut next_screen: ResMut<NextState<Screen>>,
    scene_spawner: Res<SceneSpawner>,
    scene_instances: Query<&SceneInstance>,
    just_added_scenes: Query<(), (With<SceneRoot>, Without<SceneInstance>)>,
    just_added_meshes: Query<(), Added<Mesh3d>>,
) {
    // TODO should likely also listen on PipelineCache being fully built
    // (https://github.com/bevyengine/bevy/blob/f47b1c00ee6c55f98f1858db6d6bc1fc1a4bed0e/examples/games/loading_screen.rs#L299)
    info!("Added scenes: {}", just_added_scenes.iter().count());
    if just_added_scenes.iter().count() > 0 || just_added_meshes.iter().count() > 0 {
        return;
    }
    for scene_instance in scene_instances.iter() {
        if !scene_spawner.instance_is_ready(**scene_instance) {
            return;
        }
    }
    info!("level loaded -> GamePlay");
    next_screen.set(Screen::Gameplay);
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::LoadLevel), spawn_loading_screen);
    app.add_systems(OnEnter(Screen::LoadLevel), spawn_level);
    app.add_systems(
        Update,
        monitor_load_completion.run_if(in_state(Screen::LoadLevel)),
    );
}
