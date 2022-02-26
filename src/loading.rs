use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::AppState;

pub struct LoadingPlugin;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default: Handle<Font>,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(AppState::Loading)
            .continue_to_state(AppState::Menu)
            .with_collection::<FontAssets>()
            .build(app);
    }
}
