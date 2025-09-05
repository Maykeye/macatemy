use approx::assert_abs_diff_eq;
use bevy::prelude::*;

use crate::{
    game_map_plugin::{GameMapLayerRenderer, GameMapPlugin},
    game_state_plugin::GameStatePlugin,
    player_control_plugin::PlayerControlPlugin,
    player_input_stage::PlayerInputStagesPlugin,
    test_utils::{get_position, make_defaullt_plugins_for_headless_test, press_key, release_key},
};

/// Prepare test for pressing <, > on a game map:
/// init a game map,
/// update a frame so init systems are called
/// and return app and the entity responsible for ground layer renderer
fn setup_test() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins((
        make_defaullt_plugins_for_headless_test(),
        GameStatePlugin,
        GameMapPlugin,
        PlayerInputStagesPlugin,
        PlayerControlPlugin,
    ));

    app.update();
    app.update();
    let renderer = app
        .world_mut()
        .query_filtered::<Entity, With<GameMapLayerRenderer>>()
        .single(&app.world())
        .unwrap();
    (app, renderer)
}

/// Press the key, run frame,
/// then release th key and run frame
fn press_then_release(app: &mut App, keycode: KeyCode) {
    press_key(app, keycode);
    app.update();
    release_key(app, keycode);
    app.update();
}

#[test]
fn layer_view_shift_goes_up_to_the_limit() {
    let (mut app, renderer) = setup_test();

    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);

    press_key(&mut app, KeyCode::ShiftRight);
    press_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -1.0);
    release_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -1.0);

    press_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);
    release_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);
}

#[test]
fn layer_view_shift_tries_to_go_up_beyond_limit() {
    let (mut app, renderer) = setup_test();

    // Move up
    press_key(&mut app, KeyCode::ShiftRight);
    press_then_release(&mut app, KeyCode::Comma);
    press_then_release(&mut app, KeyCode::Comma);
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);

    // Ensure limit
    press_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);
    release_key(&mut app, KeyCode::Comma);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);
}

#[test]
fn layer_view_shift_goes_bottom_from_top() {
    let (mut app, renderer) = setup_test();

    // Move up
    press_key(&mut app, KeyCode::ShiftRight);
    press_then_release(&mut app, KeyCode::Comma);
    press_then_release(&mut app, KeyCode::Comma);
    assert_abs_diff_eq!(get_position(&app, renderer).y, -2.0);

    press_key(&mut app, KeyCode::Period);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, -1.0);
    release_key(&mut app, KeyCode::Period);
    assert_abs_diff_eq!(get_position(&app, renderer).y, -1.0);

    press_key(&mut app, KeyCode::Period);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);
    release_key(&mut app, KeyCode::Period);
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);
}

#[test]
fn layer_view_shift_tries_to_go_down_beyond_limit() {
    let (mut app, renderer) = setup_test();

    press_key(&mut app, KeyCode::Period);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);

    release_key(&mut app, KeyCode::Period);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);
    app.update();
    assert_abs_diff_eq!(get_position(&app, renderer).y, 0.0);
}
