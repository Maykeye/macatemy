use bevy::{platform::collections::HashSet, prelude::*};

use crate::{
    game_map_plugin::{GameMapCellFloor, GameMapData, GameMapLayerRenderer},
    game_state_plugin::GameStatePlugin,
    test_utils::{get_resource, make_defaullt_plugins_for_headless_test, rgb_max_avg_delta},
};

use super::GameMapPlugin;

fn make_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        make_defaullt_plugins_for_headless_test(),
        GameStatePlugin,
        GameMapPlugin,
    ));
    app
}

#[test]
fn test_dummy_map_dimension() {
    let mut app = make_app();
    app.update();
    let map_data = get_resource::<GameMapData>(&app);

    assert_eq!(map_data.map.cells.len(), map_data.map.layers);
    assert!(map_data.map.width > 0);
    assert!(map_data.map.height > 0);
    for layer_idx in 0..map_data.map.layers {
        assert_eq!(map_data.map.cells[layer_idx].len(), map_data.map.height);
        for row_idx in 0..map_data.map.height {
            assert_eq!(
                map_data.map.cells[layer_idx][row_idx].len(),
                map_data.map.width
            );
        }
    }
}

#[test]
fn test_dummy_map_all_floor_entites_are_unique() {
    let mut app = make_app();
    app.update();
    let map_data = get_resource::<GameMapData>(&app);

    // Test entities of ground layer
    let layer = &map_data.map.cells[0];
    let mut ents = HashSet::new();
    for row in layer.iter() {
        for cell in row.iter() {
            assert_ne!(cell.floor_entity, Entity::PLACEHOLDER);
            assert!(ents.insert(cell.floor_entity));
        }
    }

    // Test entities of empty layers
    for layer in map_data.map.cells.iter().skip(1) {
        for row in layer.iter() {
            for cell in row.iter() {
                assert_eq!(cell.floor_entity, Entity::PLACEHOLDER);
            }
        }
    }
}

#[test]
fn test_dummy_map_default_renderer_renders_layer_0() {
    let mut app = make_app();
    app.update();
    {
        let map_layer = app
            .world_mut()
            .query::<&GameMapLayerRenderer>()
            .single(&app.world())
            .unwrap();
        assert_eq!(map_layer.0, 0);
    }
}

#[test]
fn test_dummy_map_some_tiles() {
    let mut app = make_app();
    app.update();

    let map_data = get_resource::<GameMapData>(&app);
    let layer = &map_data.map.cells[0];
    assert_eq!(layer[0][3].floor, GameMapCellFloor::Grass);
    assert_eq!(layer[0][3].floor, GameMapCellFloor::Grass);
    assert_eq!(layer[1][3].floor, GameMapCellFloor::Stone);
    assert_eq!(layer[2][3].floor, GameMapCellFloor::Ground);
    assert_eq!(layer[3][0].floor, GameMapCellFloor::Ground);
    assert_eq!(layer[3][1].floor, GameMapCellFloor::Ground);
    assert_eq!(layer[3][2].floor, GameMapCellFloor::Stone);
    assert_eq!(layer[3][3].floor, GameMapCellFloor::Stone);
    assert_eq!(layer[3][4].floor, GameMapCellFloor::Ground);
    assert_eq!(layer[4][3].floor, GameMapCellFloor::Ground);
    assert_eq!(layer[5][3].floor, GameMapCellFloor::Stone);
    assert_eq!(layer[6][3].floor, GameMapCellFloor::Grass);
    assert_eq!(layer[7][3].floor, GameMapCellFloor::Grass);
    assert_eq!(layer[8][3].floor, GameMapCellFloor::Grass);
    assert_eq!(layer[9][3].floor, GameMapCellFloor::Grass);
}

#[test]
fn test_dummy_map_floor_material() {
    let mut app = make_app();
    app.update();

    let map_data = get_resource::<GameMapData>(&app);
    let layer = &map_data.map.cells[0];
    let mats = get_resource::<Assets<StandardMaterial>>(&app);

    // Get color and max channel value
    let get_color = |row: usize, column: usize| {
        let ent = app
            .world()
            .get::<MeshMaterial3d<StandardMaterial>>(layer[row][column].floor_entity)
            .unwrap();
        let color = mats.get(ent).unwrap().base_color.to_linear();
        (color, color.red.max(color.blue).max(color.green))
    };

    let (some_grass, some_grass_max) = get_color(0, 2);
    assert_eq!(some_grass.green, some_grass_max);
    let (some_stone, _some_stone_max) = get_color(1, 2);
    assert!(rgb_max_avg_delta(some_stone) < 0.05, "not gray enough");
    let (some_ground, some_ground_max) = get_color(2, 2);
    assert_eq!(some_ground.red, some_ground_max);
}

#[cfg(test)]
#[path = "test_game_map_plugin_layer_view_shift.rs"]
mod test_game_map_plugin;
