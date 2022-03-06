use crate::loading::SpritesheetAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn render_map(
    mut commands: Commands,
    spritesheet_assets: Res<SpritesheetAssets>,
    mut map_query: MapQuery,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("spritesheets/grass.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(160.0, 128.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());

    layer_builder.fill(
        TilePos(0, 0),
        TilePos(10, 10),
        Tile {
            texture_index: 0,
            ..Default::default()
        }
        .into(),
    );

    let layer_entity = map_query.build_layer(
        &mut commands,
        layer_builder,
        texture_handle,
    );

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(GlobalTransform::default());
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_map));
    }
}
