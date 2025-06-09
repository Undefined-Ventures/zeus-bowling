use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    text::TextColor,
    ui::{AlignItems, JustifyContent, Node, UiRect, Val},
};
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{asset_tracking::LoadResource, scenes::LevelData, screens::Screen};

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
pub struct HudAssets {
    #[dependency]
    temple_image: Handle<Image>,
    #[dependency]
    skull_image: Handle<Image>,
    #[dependency]
    balling_ball_image: Handle<Image>,
}

impl FromWorld for HudAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let temple_image = asset_server.load_with_settings(
            "images/temple.png",
            |settings: &mut ImageLoaderSettings| {
                // Need to use nearest filtering to avoid bleeding between the slices with tiling
                settings.sampler = ImageSampler::nearest();
            },
        );
        let skull_image = asset_server.load_with_settings(
            "images/skull.png",
            |settings: &mut ImageLoaderSettings| {
                // Need to use nearest filtering to avoid bleeding between the slices with tiling
                settings.sampler = ImageSampler::nearest();
            },
        );
        let balling_ball_image = asset_server.load_with_settings(
            "images/balling_ball.png",
            |settings: &mut ImageLoaderSettings| {
                // Need to use nearest filtering to avoid bleeding between the slices with tiling
                settings.sampler = ImageSampler::nearest();
            },
        );
        Self {
            temple_image,
            skull_image,
            balling_ball_image,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct TempleHealthUi;

#[derive(Component, Clone, Copy)]
pub struct BallThrowsLeftUI;

#[derive(Component, Clone, Copy)]
pub struct BallTimerUI;

#[derive(Component, Clone, Copy)]
pub struct KillCountUI;

fn update_temple_health(
    level_data: Res<LevelData>,
    temple_text: Single<&mut Text, With<TempleHealthUi>>,
) {
    let mut text = temple_text.into_inner();
    *text = Text::new(format!(" {}", level_data.temple_health));
}

fn update_kill_count(level_data: Res<LevelData>, text: Single<&mut Text, With<KillCountUI>>) {
    let mut text = text.into_inner();
    *text = Text::new(format!("{}", level_data.kill_count));
}

fn update_ball_count(
    level_data: Res<LevelData>,
    count: Single<&mut Text, With<BallThrowsLeftUI>>,
    timer: Single<&mut Text, (With<BallTimerUI>, Without<BallThrowsLeftUI>)>,
) {
    let mut count_text = count.into_inner();
    *count_text = Text::new(format!("{}", level_data.balls_left));

    let mut timer_text = timer.into_inner();
    *timer_text = Text::new(format!(
        "  ({:.2})s",
        level_data.time_to_new_ball.as_secs_f32()
    ));
}

fn spawn_hud_elements(mut commands: Commands, hud_assets: Res<HudAssets>) {
    commands.spawn((
        Node {
            width: Val::Px(800.0),
            height: Val::Px(200.0),
            left: Val::Px(20.0),
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(20.0)),
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexEnd,
            ..default()
        },
        StateScoped(Screen::Gameplay),
        children![
            // Temple health
            ImageNode {
                image: hud_assets.temple_image.clone(),
                ..default()
            },
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                TempleHealthUi,
            ),
            (Text::new("        "),), // spacer
            // Kill count
            ImageNode {
                image: hud_assets.skull_image.clone(),
                ..default()
            },
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                KillCountUI,
            ),
            (Text::new("        "),), // spacer
            // Ball count
            ImageNode {
                image: hud_assets.balling_ball_image.clone(),
                ..default()
            },
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                BallThrowsLeftUI,
            ),
            (
                Text::new("  "),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                BallTimerUI,
            ),
        ],
    ));
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<HudAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud_elements);
    app.add_systems(
        Update,
        update_temple_health.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, update_kill_count.run_if(in_state(Screen::Gameplay)));
    app.add_systems(Update, update_ball_count.run_if(in_state(Screen::Gameplay)));
}
