use crate::animation::{Animation, AnimationValue};
use crate::health::{increase_health, GameStats};
use crate::loading::SpritesheetAssets;
use crate::player::Player;
use crate::AppState;
use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AnimalAnimation {
    Idle = 0,
    Eating = 1,
}

#[derive(Component, Debug)]
struct Animal;

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

    let mut rng = rand::thread_rng();
    let x: i8 = rng.gen();
    let y: i8 = rng.gen();

    let translation = Vec3::new(x.into(), y.into(), crate::z::ANIMAL);

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
        .insert(Animal)
        .insert(RigidBody::Static)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(crate::physics::Layer::Animal)
                .with_mask(crate::physics::Layer::Player),
        );
}

fn animal_player_collision(
    mut events: EventReader<CollisionEvent>,
    mut query: Query<&mut Player>,
    mut animal_query: Query<&mut Transform, With<Animal>>,
    mut game_stats: ResMut<GameStats>,
) {
    for event in events.iter() {
        if event.is_started() {
            // let (_entity_x, _entity_y) = event.rigid_body_entities();
            let (layers_x, layers_y) = event.collision_layers();
            if is_animal(layers_x) && is_player(layers_y) {
                if let Ok(mut player) = query.get_single_mut() {
                    increase_health(&mut player, &mut game_stats);
                }
                if let Ok(mut transform) = animal_query.get_single_mut() {
                    let mut rng = rand::thread_rng();
                    let x: i8 = rng.gen();
                    let y: i8 = rng.gen();
                    transform.translation.x = x.into();
                    transform.translation.y = y.into();
                }
            }
            if is_player(layers_x) && is_animal(layers_y) {
                if let Ok(mut player) = query.get_single_mut() {
                    increase_health(&mut player, &mut game_stats);
                }
                if let Ok(mut transform) = animal_query.get_single_mut() {
                    let mut rng = rand::thread_rng();
                    let x: i8 = rng.gen();
                    let y: i8 = rng.gen();
                    transform.translation.x = x.into();
                    transform.translation.y = y.into();
                }
            }
        }
    }
}

fn is_animal(layers: CollisionLayers) -> bool {
    layers.contains_group(crate::physics::Layer::Animal)
        && !layers.contains_group(crate::physics::Layer::Player)
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(crate::physics::Layer::Player)
        && !layers.contains_group(crate::physics::Layer::Animal)
}

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_animal))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(animal_player_collision),
            );
    }
}
