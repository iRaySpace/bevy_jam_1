use bevy::{input::system::exit_on_esc_system, prelude::*};

mod animation;
mod input;
mod loading;
mod menu;
mod player;

use crate::animation::AnimationPlugin;
use crate::input::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

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
            .add_plugin(AnimationPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(PlayerPlugin)
            .add_system(exit_on_esc_system);
    }
}
