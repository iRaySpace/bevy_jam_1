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

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let map_size = MapSize(2, 2);
    let chunk_size = ChunkSize(32, 32);
    let tile_size = TileSize(16., 16.);

    let layer_settings = LayerSettings::new(map_size, chunk_size, tile_size, TextureSize(64., 16.));

    let (mut layer_water_builder, _) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);
    layer_water_builder.set_all(TileBundle::default());

    let (mut layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(map_size, chunk_size, tile_size, TextureSize(160., 128.)),
        0u16,
        1u16,
    );

    // main fill
    layer_builder.fill(
        TilePos(16, 16),
        TilePos(48, 48),
        Tile {
            texture_index: 0,
            ..Default::default()
        }
        .into(),
    );

    // edges
    for x in 16..48 {
        layer_builder
            .set_tile(
                TilePos(x, 47),
                Tile {
                    texture_index: 32,
                    ..Default::default()
                }
                .into(),
            )
            .unwrap();
        layer_builder
            .set_tile(
                TilePos(x, 16),
                Tile {
                    texture_index: 52,
                    ..Default::default()
                }
                .into(),
            )
            .unwrap();
    }

    for x in 16..48 {
        layer_builder
            .set_tile(
                TilePos(16, x),
                Tile {
                    texture_index: 41,
                    ..Default::default()
                }
                .into(),
            )
            .unwrap();
        layer_builder
            .set_tile(
                TilePos(47, x),
                Tile {
                    texture_index: 43,
                    ..Default::default()
                }
                .into(),
            )
            .unwrap();
    }

    // corners
    layer_builder
        .set_tile(
            TilePos(16, 16),
            Tile {
                texture_index: 51,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();
    layer_builder
        .set_tile(
            TilePos(16, 47),
            Tile {
                texture_index: 31,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();
    layer_builder
        .set_tile(
            TilePos(47, 47),
            Tile {
                texture_index: 33,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();
    layer_builder
        .set_tile(
            TilePos(47, 16),
            Tile {
                texture_index: 53,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();

    let layer_entity = map_query.build_layer(
        &mut commands,
        layer_builder,
        spritesheet_assets.grass.clone(),
    );
    let layer_water_entity = map_query.build_layer(
        &mut commands,
        layer_water_builder,
        spritesheet_assets.water.clone(),
    );

    map.add_layer(&mut commands, 1u16, layer_entity);
    map.add_layer(&mut commands, 0u16, layer_water_entity);

    let center = layer_settings.get_pixel_center();
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-center.x, -center.y, crate::z::GROUND))
        .insert(GlobalTransform::default());
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_map));
    }
}
