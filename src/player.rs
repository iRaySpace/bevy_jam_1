use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn render_player(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = spritesheet_assets.character.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(48.0, 48.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_player))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(animate_sprite));
    }
}
