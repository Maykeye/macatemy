use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow};

pub struct PlayerControlPlugin;

fn spawn_camera(mut commands: Commands) {
    let tr = Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Name::new("Player Camera"),
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

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, player_look);
    }
}
