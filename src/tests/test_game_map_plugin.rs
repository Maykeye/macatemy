use bevy::{ecs::query, platform::collections::HashSet, prelude::*};

use crate::{
    game_map_plugin::{GameMapCellFloor, GameMapLayer},
    test_utils::{get_resource, make_defaullt_plugins_for_headless_test},
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
fn test_dummy_map_floor() {
    let mut app = make_app();
    app.update();
    let map_layer = app
        .world_mut()
        .query::<&GameMapLayer>()
        .single(&app.world())
        .unwrap();
    assert!(matches!(
        map_layer.rows[1][0].floor,
        GameMapCellFloor::Ground
    ));
    assert!(matches!(
        map_layer.rows[0][0].floor,
        GameMapCellFloor::Grass
    ));
    assert!(matches!(
        map_layer.rows[4][0].floor,
        GameMapCellFloor::Grass
    ));
    assert!(matches!(
        map_layer.rows[5][3].floor,
        GameMapCellFloor::Grass
    ));
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
    let some_ground = app
        .world()
        .get::<MeshMaterial3d<StandardMaterial>>(map_layer.rows[1][0].floor_entity)
        .unwrap();
    let color = mats.get(some_ground).unwrap().base_color.to_linear();
    let max = color.red.max(color.blue).max(color.green);
    assert_eq!(max, color.red);

    let some_grass = app
        .world()
        .get::<MeshMaterial3d<StandardMaterial>>(map_layer.rows[4][4].floor_entity)
        .unwrap();
    let color = mats.get(some_grass).unwrap().base_color.to_linear();
    let max = color.red.max(color.blue).max(color.green);
    assert_eq!(max, color.green);
}
