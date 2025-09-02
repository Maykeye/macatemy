mod inspector_plugin;
mod light_plugin;
mod player_control_plugin;
use bevy::prelude::*;
use inspector_plugin::InspectorPlugin;
use light_plugin::LightPlugin;
use player_control_plugin::PlayerControlPlugin;
use player_input_stage::PlayerInputPreUpdate;
mod player_input_stage;

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });
    commands.spawn((
        Name::new("TestCube"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(mesh),
        MeshMaterial3d(material),
    ));
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PlayerInputPreUpdate, // stage is plugin itself
        LightPlugin,
        PlayerControlPlugin,
        InspectorPlugin,
    ));
    app.add_systems(Startup, spawn_map);
    app.run();
}
