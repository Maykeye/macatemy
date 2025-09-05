use approx::assert_abs_diff_eq;
use bevy::prelude::*;

use crate::{
    game_map_plugin::{GameMapLayerRenderer, GameMapPlugin},
    game_state_plugin::GameStatePlugin,
    player_control_plugin::PlayerControlPlugin,
    player_input_stage::PlayerInputStagesPlugin,
    test_utils::{
        BaseTestSuite, get_position, make_defaullt_plugins_for_headless_test, press_key,
        release_key,
    },
};

/// Base test suite for layer view movement
struct LayerViewShiftTestSuite {
    app: App,
    ground_renderer: Entity,
}

impl BaseTestSuite for LayerViewShiftTestSuite {
    fn app(&mut self) -> &mut App {
        &mut self.app
    }
}

impl LayerViewShiftTestSuite {
    fn new() -> Self {
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
        let ground_renderer = app
            .world_mut()
            .query_filtered::<Entity, With<GameMapLayerRenderer>>()
            .single(&app.world())
            .unwrap();

        Self {
            app,
            ground_renderer,
        }
    }

    fn press_then_release_w_assert(mut self, keycode: KeyCode, y: f32) -> Self {
        press_key(&mut self.app, keycode);
        self.app.update();
        assert_abs_diff_eq!(get_position(&self.app, self.ground_renderer).y, y);
        release_key(&mut self.app, keycode);
        self.app.update();
        assert_abs_diff_eq!(get_position(&self.app, self.ground_renderer).y, y);
        self
    }
    fn assert_ground_layer_renderer_y(self, y: f32) -> Self {
        assert_abs_diff_eq!(get_position(&self.app, self.ground_renderer).y, y);
        self
    }
}

#[test]
fn layer_view_shift_goes_up() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftLeft)
        .assert_ground_layer_renderer_y(0.0)
        .press_then_release_w_assert(KeyCode::Comma, -1.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0); // limit;
}
#[test]
fn layer_view_going_up_ignores_continous_holding() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftRight)
        .press(KeyCode::Comma)
        .update()
        .update()
        .assert_ground_layer_renderer_y(-1.0);
}

#[test]
fn layer_view_shift_goes_down() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftLeft)
        .press_then_release_w_assert(KeyCode::Comma, -1.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0)
        .press_then_release_w_assert(KeyCode::Period, -1.0)
        .press_then_release_w_assert(KeyCode::Period, 0.0)
        .press_then_release_w_assert(KeyCode::Period, 0.0); // limit
}

#[test]
fn layer_view_going_down_ignores_continous_holding() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftRight)
        .press_then_release_w_assert(KeyCode::Comma, -1.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0)
        .press(KeyCode::Period)
        .update()
        .assert_ground_layer_renderer_y(-1.0)
        .update()
        .assert_ground_layer_renderer_y(-1.0);
}

#[test]
fn layer_view_shift_goes_up_shift_required() {
    LayerViewShiftTestSuite::new().press_then_release_w_assert(KeyCode::Comma, 0.0);
}

#[test]
fn layer_view_shift_goes_down_shift_required() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftLeft)
        .press_then_release_w_assert(KeyCode::Comma, -1.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0)
        .release(KeyCode::ShiftLeft)
        .press_then_release_w_assert(KeyCode::Comma, -2.0);
}

#[test]
fn layer_view_shift_goes_up_shift_must_be_pressed_first() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::Comma)
        .assert_ground_layer_renderer_y(0.0)
        .press(KeyCode::ShiftRight)
        .assert_ground_layer_renderer_y(0.0);
}

#[test]
fn layer_view_shift_goes_down_shift_must_be_pressed_first() {
    LayerViewShiftTestSuite::new()
        .press(KeyCode::ShiftRight)
        .press_then_release_w_assert(KeyCode::Comma, -1.0)
        .press_then_release_w_assert(KeyCode::Comma, -2.0)
        .release(KeyCode::ShiftRight)
        .press_then_release_w_assert(KeyCode::Period, -2.0);
}
