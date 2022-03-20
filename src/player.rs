use crate::animation::{Animation, AnimationValue};
use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Debug, PartialEq)]
pub enum PlayerStateMachine {
    Idle,
    Moving,
}

#[derive(Debug, PartialEq)]
pub enum PlayerAnimation {
    UpMoving = 0,
    RightMoving = 1,
    DownMoving = 2,
    LeftMoving = 3,
    UpIdle = 4,
    RightIdle = 5,
    DownIdle = 6,
    LeftIdle = 7,
    None,
}

// TODO: Some or None
impl PlayerAnimation {
    fn from_u8(val: u8) -> PlayerAnimation {
        match val {
            0 => PlayerAnimation::UpMoving,
            1 => PlayerAnimation::RightMoving,
            2 => PlayerAnimation::DownMoving,
            3 => PlayerAnimation::LeftMoving,
            4 => PlayerAnimation::UpIdle,
            5 => PlayerAnimation::RightIdle,
            6 => PlayerAnimation::DownIdle,
            7 => PlayerAnimation::LeftIdle,
            _ => PlayerAnimation::None,
        }
    }
    fn build(self) -> AnimationValue {
        match self {
            PlayerAnimation::UpMoving => AnimationValue {
                frames: vec![4, 5, 6, 7],
            },
            PlayerAnimation::RightMoving => AnimationValue {
                frames: vec![12, 13, 14, 15],
            },
            PlayerAnimation::DownMoving => AnimationValue {
                frames: vec![0, 1, 2, 3],
            },
            PlayerAnimation::LeftMoving => AnimationValue {
                frames: vec![8, 9, 10, 11],
            },
            PlayerAnimation::UpIdle => AnimationValue { frames: vec![4] },
            PlayerAnimation::RightIdle => AnimationValue { frames: vec![12] },
            PlayerAnimation::DownIdle => AnimationValue { frames: vec![0] },
            PlayerAnimation::LeftIdle => AnimationValue { frames: vec![8] },
            PlayerAnimation::None => AnimationValue { frames: vec![] },
        }
    }
    fn next(self) -> PlayerAnimation {
        match self {
            PlayerAnimation::UpMoving => PlayerAnimation::UpIdle,
            PlayerAnimation::RightMoving => PlayerAnimation::RightIdle,
            PlayerAnimation::DownMoving => PlayerAnimation::DownIdle,
            PlayerAnimation::LeftMoving => PlayerAnimation::LeftIdle,
            PlayerAnimation::UpIdle => PlayerAnimation::UpIdle,
            PlayerAnimation::RightIdle => PlayerAnimation::RightIdle,
            PlayerAnimation::DownIdle => PlayerAnimation::DownIdle,
            PlayerAnimation::LeftIdle => PlayerAnimation::LeftIdle,
            PlayerAnimation::None => PlayerAnimation::None,
        }
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
    pub state: PlayerStateMachine,
    pub grass: u8,
}

fn render_player(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2::new(48., 48.);

    let texture = spritesheet_assets.character.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, size, 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let translation = Vec3::new(0., 0., crate::z::PLAYER);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)).with_translation(translation),
            ..Default::default()
        })
        .insert(Animation {
            values: vec![
                PlayerAnimation::build(PlayerAnimation::UpMoving),
                PlayerAnimation::build(PlayerAnimation::RightMoving),
                PlayerAnimation::build(PlayerAnimation::DownMoving),
                PlayerAnimation::build(PlayerAnimation::LeftMoving),
                PlayerAnimation::build(PlayerAnimation::UpIdle),
                PlayerAnimation::build(PlayerAnimation::RightIdle),
                PlayerAnimation::build(PlayerAnimation::DownIdle),
                PlayerAnimation::build(PlayerAnimation::LeftIdle),
            ],
            current_value: PlayerAnimation::DownIdle as u8,
            current_frame: 0,
            duration: Timer::from_seconds(0.2, true),
        })
        .insert(Player {
            speed: 200.,
            state: PlayerStateMachine::Idle,
            grass: 0,
        })
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(24.0, 24.0).extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(Velocity::default())
        .insert(
            CollisionLayers::none()
                .with_group(crate::physics::Layer::Player)
                .with_mask(crate::physics::Layer::Consumable)
                .with_mask(crate::physics::Layer::Animal),
        );
}

fn read_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Animation, &mut Velocity)>,
) {
    if let Ok((mut player, mut animation, mut velocity)) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            player.state = PlayerStateMachine::Moving;
            animation.current_value = PlayerAnimation::UpMoving as u8;
            velocity.linear = Vec2::new(0.0, 1.0).extend(0.0) * player.speed;
        } else if keyboard_input.pressed(KeyCode::Down) {
            player.state = PlayerStateMachine::Moving;
            animation.current_value = PlayerAnimation::DownMoving as u8;
            velocity.linear = Vec2::new(0.0, -1.0).extend(0.0) * player.speed;
        } else if keyboard_input.pressed(KeyCode::Left) {
            player.state = PlayerStateMachine::Moving;
            animation.current_value = PlayerAnimation::LeftMoving as u8;
            velocity.linear = Vec2::new(-1.0, 0.0).extend(0.0) * player.speed;
        } else if keyboard_input.pressed(KeyCode::Right) {
            player.state = PlayerStateMachine::Moving;
            animation.current_value = PlayerAnimation::RightMoving as u8;
            velocity.linear = Vec2::new(1.0, 0.0).extend(0.0) * player.speed;
        } else {
            player.state = PlayerStateMachine::Idle;
            animation.current_value =
                PlayerAnimation::next(PlayerAnimation::from_u8(animation.current_value)) as u8;
            velocity.linear = Vec2::new(0.0, 0.0).extend(0.0);
        }
    }
}

fn player_consumable_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut query: Query<&mut Player>,
) {
    for event in events.iter() {
        if event.is_started() {
            let (entity_x, entity_y) = event.rigid_body_entities();
            let (layers_x, layers_y) = event.collision_layers();
            if is_player(layers_x) && is_consumable(layers_y) {
                commands.entity(entity_y).despawn();
                if let Ok(mut player) = query.get_single_mut() {
                    player.grass += 1;
                }
            }
            if is_consumable(layers_x) && is_player(layers_y) {
                commands.entity(entity_x).despawn();
                if let Ok(mut player) = query.get_single_mut() {
                    player.grass += 1;
                }
            }
        }
    }
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(crate::physics::Layer::Player)
        && !layers.contains_group(crate::physics::Layer::Consumable)
}

fn is_consumable(layers: CollisionLayers) -> bool {
    layers.contains_group(crate::physics::Layer::Consumable)
        && !layers.contains_group(crate::physics::Layer::Player)
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(read_input)
                    .with_system(player_consumable_collision),
            );
    }
}
