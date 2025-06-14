//! The game's main screen states and transitions between them.

pub mod asset_loading;
mod end;
mod gameplay;
mod level_loading;
mod skein_server;
mod splash;
mod title;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

/// The game's main screen states.
#[auto_init_state]
#[auto_register_state_type]
#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    LoadLevel,
    Gameplay,
    SkeinServer,
    End,
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        end::plugin,
        gameplay::plugin,
        asset_loading::plugin,
        level_loading::plugin,
        splash::plugin,
        title::plugin,
        skein_server::plugin,
    ));
}
