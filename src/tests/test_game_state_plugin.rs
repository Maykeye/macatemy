#![cfg(test)]
use bevy::{prelude::*, state::app::StatesPlugin};

use crate::test_utils::{get_resource, is_entity_alive};

use super::{GameObject, GameState, GameStatePlugin};

#[derive(Resource, Default)]
struct UninitCounter(u32);
#[derive(Resource, Default)]
struct InitCounter(u32);

fn make_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, GameStatePlugin));
    app.init_resource::<UninitCounter>();
    app.init_resource::<InitCounter>();
    app
}

#[test]
fn test_game_states_order() {
    let mut app = make_app();

    // Make sure that init is the first state
    app.update();
    {
        let state = app.world().get_resource::<State<GameState>>().unwrap();
        assert_eq!(*state, GameState::Init);
    }

    // Make sure that next frames are in Game state
    const GAME_FRAMES: usize = 10;
    for _ in 0..GAME_FRAMES {
        app.update();
        {
            let state = app.world().get_resource::<State<GameState>>().unwrap();
            assert_eq!(*state, GameState::Game);
        }
    }
}

// Switch state to uninit in 3 frames
fn switch_to_uninit_in_three_frames(
    mut next: ResMut<NextState<GameState>>,
    mut counter: Local<usize>,
) {
    const THRESHOLD: usize = 3;
    let next_counter = *counter + 1;
    if next_counter < THRESHOLD {
        *counter = next_counter;
    } else if next_counter == THRESHOLD {
        next.set(GameState::Uninit);
        *counter = next_counter + 1;
    } else {
        // do nothing
    }
}
fn run_in_init(mut ctr: ResMut<InitCounter>) {
    ctr.0 += 1;
}
fn run_in_uninit(mut ctr: ResMut<UninitCounter>) {
    ctr.0 += 1;
}

fn run_in_uninit_for_obj(q: Query<Entity, With<GameObject>>, mut ctr: ResMut<UninitCounter>) {
    ctr.0 += q.iter().count() as u32;
}
#[test]
fn test_game_states_uninit_goes_to_init() {
    let mut app = make_app();

    // "The StateTransition schedule runs after PreUpdate (which contains Bevy engine internals),
    // but before FixedMain (fixed timestep) and Update, where your game's systems usually live."

    // Add usual system that will change the state
    app.add_systems(
        Update,
        (
            switch_to_uninit_in_three_frames.run_if(in_state(GameState::Game)),
            run_in_uninit.run_if(in_state(GameState::Uninit)),
            run_in_init.run_if(in_state(GameState::Init)),
        ),
    );

    // Init frame
    // Execute system attached to GameState::Init
    app.update();
    {
        let state = get_resource::<State<GameState>>(&app);
        assert_eq!(*state, GameState::Init);
        let state = get_resource::<InitCounter>(&app);
        assert_eq!(state.0, 1);
    }

    // Game frames:
    // Execute 3 frames within GameState::Game
    app.update(); // NOP
    app.update(); // NOP
    app.update(); // Schedule Uninit after scene transition is done(between PreUpdate and Update)
    {
        let state = get_resource::<State<GameState>>(&app);
        assert_eq!(*state, GameState::Game);
        let state = get_resource::<UninitCounter>(&app);
        assert_eq!(state.0, 0);
        let state = get_resource::<InitCounter>(&app);
        assert_eq!(state.0, 1);
    }

    // Before update, translate to scene to Uninit, in update run Uninit scene which schedules Init
    // for the next frame
    app.update();
    {
        let state = get_resource::<State<GameState>>(&app);
        assert_eq!(*state, GameState::Uninit);
        let state = get_resource::<UninitCounter>(&app);
        assert_eq!(state.0, 1);
        let state = get_resource::<InitCounter>(&app);
        assert_eq!(state.0, 1);
    }

    // Runs scheduled init state
    app.update();
    {
        let state = get_resource::<State<GameState>>(&app);
        assert_eq!(*state, GameState::Init);
        let state = get_resource::<UninitCounter>(&app);
        assert_eq!(state.0, 1);
        let state = get_resource::<InitCounter>(&app);
        assert_eq!(state.0, 2);
    }

    // Just game frames, no transitions are done
    for _ in 0..10 {
        app.update();
        {
            let state = get_resource::<State<GameState>>(&app);
            assert_eq!(*state, GameState::Game);
            let state = get_resource::<UninitCounter>(&app);
            assert_eq!(state.0, 1);
            let state = get_resource::<InitCounter>(&app);
            assert_eq!(state.0, 2);
        }
    }
}

#[test]
fn test_uninit_auto_despawn_game_objects() {
    let mut app = make_app();
    app.add_systems(
        Update,
        (
            switch_to_uninit_in_three_frames.run_if(in_state(GameState::Game)),
            run_in_uninit_for_obj.run_if(in_state(GameState::Uninit)),
        ),
    );

    let ent: Entity = app.world_mut().spawn(GameObject).id();

    // Init
    app.update();

    assert!(is_entity_alive(&app, ent));

    app.update(); // NOP
    app.update(); // NOP
    app.update(); // Schedule Uninit after scene transition is done(between PreUpdate and Update)
    assert!(is_entity_alive(&app, ent));
    {
        let state = get_resource::<UninitCounter>(&app);
        assert_eq!(state.0, 0);
    }

    // Before update, translate to scene to Uninit, in update run Uninit scene which deletes
    // GameObjects
    app.update();
    assert!(!is_entity_alive(&app, ent));
    {
        let state = get_resource::<UninitCounter>(&app);
        assert_eq!(state.0, 1);
    }
}
