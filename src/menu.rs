use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::AppState;

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.80, 0.49, 0.12).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.default.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn btn_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut UiColor), With<Button>>,
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn_recursive();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.72, 0.40, 0.09).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.80, 0.49, 0.12).into();
            }
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(btn_system));
    }
}
