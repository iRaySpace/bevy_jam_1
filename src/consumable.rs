use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Consumable {
    Glass = 0,
    GlassHalfMilk = 1,
    GlassFullMilk = 2,
    Grass = 3,
}

struct ConsumableSpawnTimer(Timer);

fn spawn_consumable(
    mut timer: ResMut<ConsumableSpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x: i8 = rng.gen();
        let y: i8 = rng.gen();
        let size = Vec2::new(16., 16.);

        let texture = spritesheet_assets.milk_grass.clone();
        let texture_atlas = TextureAtlas::from_grid(texture, size, 4, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(
                    x.into(),
                    y.into(),
                    crate::z::CONSUMABLES,
                )),
                sprite: TextureAtlasSprite {
                    index: Consumable::Grass as usize,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(crate::physics::Layer::Consumable)
                    .with_mask(crate::physics::Layer::Player),
            );
    }
}

pub struct ConsumablePlugin;

impl Plugin for ConsumablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsumableSpawnTimer(Timer::from_seconds(2.0, true)))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_consumable));
    }
}
