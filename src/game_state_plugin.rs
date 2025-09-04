use bevy::prelude::*;

pub struct GameStatePlugin;

/// GameObject as an object that will be automatically despawned
/// during Uninit phase in PostUpdate stage.
/// If you need to work with objects unitialization intact, use Update stage
#[derive(Component)]
pub struct GameObject;

#[derive(Debug, States, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    /// Initialize a game
    #[default]
    Init,
    /// Play
    Game,
    /// Destroy resources added in cleanup
    Uninit,
}

fn switch_init_to_play(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Game);
}
fn switch_uninit_to_init(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Init);
}

fn despawn_game_objects_on_uninit(q: Query<Entity, With<GameObject>>, mut cmds: Commands) {
    for entity in q {
        cmds.entity(entity).despawn();
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(
            Update,
            (
                switch_init_to_play.run_if(in_state(GameState::Init)),
                switch_uninit_to_init.run_if(in_state(GameState::Uninit)),
            ),
        );
        app.add_systems(
            PostUpdate,
            despawn_game_objects_on_uninit.run_if(in_state(GameState::Uninit)),
        );
    }
}

#[cfg(test)]
#[path = "./tests/test_game_state_plugin.rs"]
mod test_game_state_plugin;
