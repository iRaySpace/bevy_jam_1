use bevy::{input::system::exit_on_esc_system, prelude::*};

mod animation;
mod consumable;
mod loading;
mod map;
mod menu;
mod physics;
mod player;
mod z;

use crate::animation::AnimationPlugin;
use crate::consumable::ConsumablePlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
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
            .add_plugin(PhysicsPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ConsumablePlugin)
            .add_system(exit_on_esc_system);
    }
}
