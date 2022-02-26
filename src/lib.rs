use bevy::{input::system::exit_on_esc_system, prelude::*};

mod input;
mod loading;
mod menu;

use crate::input::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state(AppState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InputPlugin)
            .add_system(exit_on_esc_system);
    }
}
