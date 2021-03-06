use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::AppState;

pub struct LoadingPlugin;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct SpritesheetAssets {
    #[asset(path = "spritesheets/character.png")]
    pub character: Handle<Image>,

    #[asset(path = "spritesheets/grass.png")]
    pub grass: Handle<Image>,

    #[asset(path = "spritesheets/water.png")]
    pub water: Handle<Image>,

    #[asset(path = "spritesheets/milk_grass.png")]
    pub milk_grass: Handle<Image>,

    #[asset(path = "spritesheets/calf.png")]
    pub calf: Handle<Image>,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(AppState::Loading)
            .continue_to_state(AppState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<SpritesheetAssets>()
            .build(app);
    }
}
