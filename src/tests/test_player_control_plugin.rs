use std::time::Duration;

use bevy::{input::InputPlugin, prelude::*, time::TimeUpdateStrategy};

use crate::{
    player_input_stage::PlayerInputStagesPlugin,
    test_utils::{contains_exact_event, press_key},
};

use super::{Player, PlayerControlPlugin};

#[cfg(test)]
fn make_app() -> App {
    use bevy::state::app::StatesPlugin;

    use crate::game_state_plugin::GameStatePlugin;

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        StatesPlugin,
        GameStatePlugin,
        PlayerInputStagesPlugin,
        PlayerControlPlugin,
    ));
    app
}

#[test]
fn pressing_ctrl_q_leads_to_exit() {
    let mut app = make_app();

    app.update();
    assert!(!contains_exact_event(&app, AppExit::Success));

    press_key(&mut app, KeyCode::ControlLeft);
    app.update();
    assert!(!contains_exact_event(&app, AppExit::Success));

    press_key(&mut app, KeyCode::KeyQ);
    app.update();
    assert!(contains_exact_event(&app, AppExit::Success));
}

#[test]
fn pressing_q_before_ctrl_doesnt_lead_to_exit() {
    let mut app = make_app();

    app.update();
    assert!(!contains_exact_event(&app, AppExit::Success));

    press_key(&mut app, KeyCode::KeyQ);
    app.update();
    assert!(!contains_exact_event(&app, AppExit::Success));

    press_key(&mut app, KeyCode::ControlLeft);
    app.update();
    assert!(!contains_exact_event(&app, AppExit::Success));
}

// Basic implementation of camera movement.
// Spawn a camea at a given pos, move according to a key press,
// return camera position afterwards
// Returns position relatevily to initial
fn impl_camera_move_test(initial_pos: Vec3, keycode: KeyCode) -> Vec3 {
    let mut app = make_app();
    let fps60 = 1.0 / 60.0;
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
        fps60,
    )));

    app.update();

    // Move camera to its initial pos
    {
        let mut world = app.world_mut();
        let mut player = world
            .query_filtered::<&mut Transform, With<Player>>()
            .single_mut(&mut world)
            .unwrap();

        *player = player
            .with_translation(initial_pos)
            .looking_at(Vec3::new(5.0, 0.0, 5.5), Vec3::Y);
    }

    // Schedule camera movement
    press_key(&mut app, keycode);
    app.update();

    // Move camera according to presses
    app.update();
    let player = app
        .world_mut()
        .query_filtered::<&Transform, With<Player>>()
        .single(&app.world())
        .unwrap();
    assert_eq!(initial_pos.y, player.translation.y);
    let delta = player.translation - initial_pos;
    const EPS: f32 = 0.1;
    assert!(delta.x.abs() < EPS);
    assert!(delta.y.abs() < EPS);
    assert!(delta.z.abs() < EPS);
    delta
}

#[test]
fn move_camera_forward_in_diagonal_direction() {
    let initial_pos = Vec3::new(4.0, 2.0, 10.0);
    let delta = impl_camera_move_test(initial_pos, KeyCode::KeyW);
    assert!(delta.x > 0.0);
    assert!(delta.z < 0.0);
    assert!(delta.x.abs() < delta.z.abs());

    // Inverse (note, we use MinimalPlugins so constructing anew is safe)
    let delta = impl_camera_move_test(initial_pos, KeyCode::KeyS);
    assert!(delta.x < 0.0);
    assert!(delta.z > 0.0);
    assert!(delta.x.abs() < delta.z.abs());
}

#[test]
fn move_camera_right_diagonal_direction() {
    let initial_pos = Vec3::new(4.0, 2.0, 10.0);
    let delta = impl_camera_move_test(initial_pos, KeyCode::KeyD);
    assert!(delta.x > 0.0);
    assert!(delta.z > 0.0);
    assert!(delta.x.abs() > delta.z.abs()); //Z goes down faster as X almost here

    // Inverse (note, we use MinimalPlugins so constructing anew is safe)
    let delta = impl_camera_move_test(initial_pos, KeyCode::KeyA);
    assert!(delta.x < 0.0);
    assert!(delta.z < 0.0);
    assert!(delta.x.abs() > delta.z.abs());
}
