//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{menus::Menu, screens::Screen, theme::widget};

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Show Controls", open_show_controls),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_show_controls(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::ViewControls);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}
