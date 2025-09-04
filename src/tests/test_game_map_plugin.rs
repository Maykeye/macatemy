use bevy::{platform::collections::HashSet, prelude::*};

use crate::{
    game_map_plugin::{GameMapCellFloor, GameMapLayer},
    test_utils::{get_resource, make_defaullt_plugins_for_headless_test, rgb_max_avg_delta},
};

use super::GameMapPlugin;

fn make_app() -> App {
    let mut app = App::new();
    app.add_plugins((make_defaullt_plugins_for_headless_test(), GameMapPlugin));
    app
}

#[test]
fn test_dummy_map_dimension() {
    let mut app = make_app();
    app.update();
    let map_layer = app
        .world_mut()
        .query::<&GameMapLayer>()
        .single(&app.world())
        .unwrap();
    assert_eq!(map_layer.rows.len(), 10);
    assert_eq!(map_layer.rows[0].len(), 10);
}

#[test]
fn test_dummy_map_all_floor_entites_are_unique() {
    let mut app = make_app();
    app.update();
    let map_layer = app
        .world_mut()
        .query::<&GameMapLayer>()
        .single(&app.world())
        .unwrap();

    let mut ents = HashSet::new();
    for row in map_layer.rows.iter() {
        for cell in row.iter() {
            assert!(cell.floor_entity != Entity::PLACEHOLDER);
            assert!(ents.insert(cell.floor_entity));
        }
    }
}

#[test]
fn test_dummy_map_floor_tile() {
    let mut app = make_app();
    app.update();
    let map_layer = app
        .world_mut()
        .query::<&GameMapLayer>()
        .single(&app.world())
        .unwrap();

    // Ensure road-like
    assert_eq!(map_layer.rows[0][3].floor, GameMapCellFloor::Grass);
    assert_eq!(map_layer.rows[1][3].floor, GameMapCellFloor::Stone);
    assert_eq!(map_layer.rows[2][3].floor, GameMapCellFloor::Ground);
    assert_eq!(map_layer.rows[3][0].floor, GameMapCellFloor::Ground);
    assert_eq!(map_layer.rows[3][1].floor, GameMapCellFloor::Ground);
    assert_eq!(map_layer.rows[3][2].floor, GameMapCellFloor::Stone);
    assert_eq!(map_layer.rows[3][3].floor, GameMapCellFloor::Stone);
    assert_eq!(map_layer.rows[3][4].floor, GameMapCellFloor::Ground);
    assert_eq!(map_layer.rows[4][3].floor, GameMapCellFloor::Ground);
    assert_eq!(map_layer.rows[5][3].floor, GameMapCellFloor::Stone);
    assert_eq!(map_layer.rows[6][3].floor, GameMapCellFloor::Grass);
    assert_eq!(map_layer.rows[7][3].floor, GameMapCellFloor::Grass);
    assert_eq!(map_layer.rows[8][3].floor, GameMapCellFloor::Grass);
    assert_eq!(map_layer.rows[9][3].floor, GameMapCellFloor::Grass);
}

#[test]
fn test_dummy_map_floor_material() {
    let mut app = make_app();
    app.update();
    let map_layer = app
        .world_mut()
        .query::<&GameMapLayer>()
        .single(&app.world())
        .unwrap();

    let mats = get_resource::<Assets<StandardMaterial>>(&app);

    // Get color and max channel value
    let get_color = |row: usize, column: usize| {
        let ent = app
            .world()
            .get::<MeshMaterial3d<StandardMaterial>>(map_layer.rows[row][column].floor_entity)
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
