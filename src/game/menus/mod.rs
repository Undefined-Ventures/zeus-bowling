//! The game's menus and transitions between them.

mod credits;
mod end;
mod main;
mod pause;
mod settings;
mod view_controls;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_state_type]
#[auto_init_state]
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    ViewControls,
    Settings,
    Pause,
    End,
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        credits::plugin,
        end::plugin,
        main::plugin,
        pause::plugin,
        settings::plugin,
        view_controls::plugin,
    ));
}
