use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn render_map(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = spritesheet_assets.grass.clone();

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        MapSize(2, 2),
        ChunkSize(12, 12),
        TileSize(16., 16.),
        TextureSize(160., 128.),
    );

    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);
    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);

    let center = layer_settings.get_pixel_center();
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-center.x, -center.y, 0.))
        .insert(GlobalTransform::default());
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_map));
    }
}
