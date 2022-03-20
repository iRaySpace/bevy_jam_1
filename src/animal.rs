use crate::animation::{Animation, AnimationValue};
use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AnimalAnimation {
    Idle = 0,
    Eating = 1,
}

// TODO: Some or None
impl AnimalAnimation {
    fn build(self) -> AnimationValue {
        match self {
            AnimalAnimation::Idle => AnimationValue {
                frames: vec![8, 9, 10, 11],
            },
            AnimalAnimation::Eating => AnimationValue {
                frames: vec![56, 57, 58, 59],
            },
        }
    }
}

fn render_animal(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2::new(32., 32.);

    let texture = spritesheet_assets.calf.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, size, 8, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let translation = Vec3::new(-100., -100., crate::z::ANIMAL);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)).with_translation(translation),
            ..Default::default()
        })
        .insert(Animation {
            values: vec![
                AnimalAnimation::build(AnimalAnimation::Idle),
                AnimalAnimation::build(AnimalAnimation::Eating),
            ],
            current_value: AnimalAnimation::Idle as u8,
            current_frame: 0,
            duration: Timer::from_seconds(0.2, true),
        })
        .insert(RigidBody::Static)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        });
}

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_animal));
    }
}
