use bevy::{input::InputPlugin, prelude::*};

use crate::{
    player_input_stage::PlayerInputStagesPlugin,
    test_utils::{contains_exact_event, press_key},
};

use super::PlayerControlPlugin;

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
