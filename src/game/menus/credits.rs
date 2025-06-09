//! The credits menu.

use crate::game::asset_tracking::LoadResource;
use crate::game::audio::music;
use crate::game::menus::Menu;
use crate::game::theme::widget;
use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        StateScoped(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}

fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Credits),
        TextFont::from_font_size(10.),
        children![
            widget::header("Created by"),
            created_by(),
            widget::header("Assets"),
            assets(),
            widget::header("Notable Libs"),
            libs(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        [
            "Brett Striker",
            "Programmer / 3D Art / Mad Scientist / Narnia Explorer",
        ],
        [
            "Michail Denchev",
            "Programmer / 'Can we de-scope this?' guy",
        ],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Music", "CC BY 3.0 by Kevin MacLeod"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
        [
            "Skele mesh",
            "https://initial-project.itch.io/low-poly-skeletons",
        ],
        [
            "bowling ball",
            "https://www.cgtrader.com/free-3d-models/sports/game/bowling-ball-and-pins-d513306d-a290-46e3-86f7-adb8bec45d97",
        ],
        [
            "Bone Snap Sfx 1",
            "https://pixabay.com/sound-effects/bone-snap-295399/",
        ],
        [
            "Bone Snap Sfx 2",
            "https://pixabay.com/sound-effects/bone-break-sound-269658/",
        ],
        [
            "Throw Sfx",
            "https://pixabay.com/sound-effects/whoosh-313320/",
        ],
    ])
}

fn libs() -> impl Bundle {
    grid(vec![
        ["Avian3D", "bevy-inspector-egui"],
        ["Skein", "bevy_panorbit_camera"],
        ["bevy_egui", "bevy_auto_plugin"],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}
