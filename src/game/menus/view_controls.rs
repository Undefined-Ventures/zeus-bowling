//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{menus::Menu, screens::Screen, theme::prelude::*};

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Controls"),
        GlobalZIndex(2),
        StateScoped(Menu::ViewControls),
        children![
            widget::header("Controls"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Controls Grid"),
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            widget::label("Aim: A/D"),
            widget::label("Fire: Space"),
            widget::mini_header("-- Cheats --"),
            widget::label("Change Ball Speed: Up/Down Arrow"),
        ],
    )
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::ViewControls), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::ViewControls).and(input_just_pressed(KeyCode::Escape))),
    );
}
