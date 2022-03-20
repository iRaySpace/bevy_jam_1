use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::AppState;

#[derive(Debug)]
pub struct GameStats {
    health: f32,
}

#[derive(Component)]
pub struct ScoreText;

fn render_health(mut commands: Commands, font_assets: Res<FontAssets>, game_stats: Res<GameStats>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Health: ".to_string() + &game_stats.health.to_string(),
                TextStyle {
                    font: font_assets.default.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ScoreText);
}

fn reduce_health(
    mut game_stats: ResMut<GameStats>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    game_stats.health -= time.delta_seconds();
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Health: {:.0}", game_stats.health);
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameStats { health: 100.0 })
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_health))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(reduce_health));
    }
}
