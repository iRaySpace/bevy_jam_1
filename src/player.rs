use crate::animation::{Animation, AnimationValue};
use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

fn render_player(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = spritesheet_assets.character.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(48.0, 48.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..Default::default()
        })
        .insert(Animation {
            values: vec![
                // 0: bottom
                AnimationValue {
                    frames: vec![0, 1, 2, 3],
                },
                // 1: top
                AnimationValue {
                    frames: vec![4, 5, 6, 7],
                },
                // 2: left
                AnimationValue {
                    frames: vec![8, 9, 10, 11],
                },
                // 3: right
                AnimationValue {
                    frames: vec![12, 13, 14, 15],
                },
            ],
            current_value: 0,
            current_frame: 0,
            duration: Timer::from_seconds(0.1, true),
        })
        .insert(Player);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_player));
    }
}
