use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow};

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn light
    commands.spawn((DirectionalLight::default(),));
    // Spawn cubes
    let mesh = meshes.add(Cuboid::from_length(1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(mesh),
        MeshMaterial3d(material),
    ));

    let tr = Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3d::default(),
        tr,
        Projection::Perspective(PerspectiveProjection {
            fov: 90.0f32.to_radians(),
            near: 0.1,
            far: 1000.0,
            ..default()
        }),
    ));
}

fn player_look(
    mut player: Single<&mut Transform, With<Camera3d>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }
    let dt = time.delta_secs();
    let sensetivity = 100. / window.width().min(window.height());
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(EulerRot::YXZ);

    pitch -= mouse_motion.delta.y * dt * sensetivity;
    yaw -= mouse_motion.delta.x * dt * sensetivity;
    pitch = pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
    player.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,));
    app.add_systems(Startup, spawn_map);
    app.add_systems(Update, player_look);
    app.run();
}
