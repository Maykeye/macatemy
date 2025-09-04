mod inspector_plugin;
mod light_plugin;
mod player_control_plugin;

use bevy::prelude::*;
use game_map_plugin::GameMapPlugin;
use inspector_plugin::InspectorPlugin;
use light_plugin::LightPlugin;
use player_control_plugin::PlayerControlPlugin;
use player_input_stage::PlayerInputStagesPlugin;
mod game_map_plugin;
mod player_input_stage;

#[path = "./tests/test_utils.rs"]
mod test_utils;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PlayerInputStagesPlugin,
        LightPlugin,
        GameMapPlugin,
        PlayerControlPlugin,
        InspectorPlugin,
    ));
    app.run();
}
