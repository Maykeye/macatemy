use crate::{
    game_state_plugin::{GameObject, GameState},
    player_input_stage::{PlayerInputPostUpdate, PlayerInputPreUpdate},
};
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use std::f32::consts::FRAC_PI_2;

pub struct PlayerControlPlugin;

#[derive(Debug, Clone, Copy)]
pub struct MoveCameraXZ(Vec3);

impl MoveCameraXZ {
    pub fn new(forward: f32, right: f32) -> Self {
        Self(Vec3::new(right, 0.0, forward))
    }
    pub fn forward(&self) -> f32 {
        self.0.z
    }
    pub fn right(&self) -> f32 {
        self.0.x
    }
}

#[derive(Event, Debug)]
pub enum PlayerCommand {
    QuitApp,
    MoveCameraXZ(MoveCameraXZ),
    MoveCameraInOut(f32),
}

#[derive(Component)]
pub struct Player;

fn spawn_camera(mut commands: Commands) {
    let looking_at = Vec3::new(5.0, 0.0, 5.0);
    let tr = Transform::from_xyz(0.0, 2.0, 4.0).looking_at(looking_at, Vec3::Y);
    commands.spawn((
        GameObject,
        Player,
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused || !keyboard.pressed(KeyCode::ControlLeft) {
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

fn directional_keys(input: &ButtonInput<KeyCode>, pos: KeyCode, neg: KeyCode) -> f32 {
    if input.pressed(pos) {
        1.0
    } else if input.pressed(neg) {
        -1.0
    } else {
        0.0
    }
}

fn player_keyboard_input(mut ev: EventWriter<PlayerCommand>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::AltLeft) && input.just_pressed(KeyCode::KeyQ) {
        ev.write(PlayerCommand::QuitApp);
    }

    let move_fwd = directional_keys(&input, KeyCode::KeyW, KeyCode::KeyS);
    let move_right = directional_keys(&input, KeyCode::KeyD, KeyCode::KeyA);
    if (move_fwd, move_right) != (0.0, 0.0) {
        ev.write(PlayerCommand::MoveCameraXZ(MoveCameraXZ::new(
            move_fwd, move_right,
        )));
    }
}

fn player_move_with_mouse_wheel(
    mut evs: EventWriter<PlayerCommand>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
) {
    const CAMERA_ZOOM_SPEED: f32 = 5.0;
    if mouse_wheel.delta.y == 0.0 {
        return;
    }

    let move_by = match mouse_wheel.unit {
        MouseScrollUnit::Line => mouse_wheel.delta.y,
        MouseScrollUnit::Pixel => mouse_wheel.delta.y / 16.0, // have no idea how to cause this event
    };

    evs.write(PlayerCommand::MoveCameraInOut(move_by * CAMERA_ZOOM_SPEED));
}

fn player_cmd_quit(mut evs: EventReader<PlayerCommand>, mut exit: EventWriter<AppExit>) {
    for _ in evs.read().filter(|x| matches!(x, PlayerCommand::QuitApp)) {
        exit.write(AppExit::Success);
    }
}

fn player_cmd_move_camera(
    mut evs: EventReader<PlayerCommand>,
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time<Real>>,
) {
    const CAMERA_SPEED: f32 = 3.00;
    for ev in evs.read() {
        use PlayerCommand::*;
        let transition = match ev {
            MoveCameraXZ(move_camera) => {
                let transition =
                    move_camera.forward() * player.forward() + move_camera.right() * player.right();
                transition.with_y(0.0).normalize_or_zero()
            }
            MoveCameraInOut(n) => *n * player.forward(),
            _ => continue,
        };
        let transition = transition * CAMERA_SPEED * time.delta_secs();
        player.translation += transition;
        player.translation.y = player.translation.y.clamp(1.0, 5.0);
    }
}

fn ensure_grabbed_cursor(window: &mut Window) {
    use bevy::window::CursorGrabMode;
    if window.cursor_options.grab_mode == CursorGrabMode::None {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.title = "Press ESC to release the cursor.".to_string();
    }
}

fn ensure_released_cursor(window: &mut Window) {
    use bevy::window::CursorGrabMode;
    if window.cursor_options.grab_mode != CursorGrabMode::None {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.title = "Press ESC to grab the cursor.".to_string();
    }
}

fn grab_focused_window(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut discard_grabbing: Local<bool>,
) {
    // TODO: should we prevent reacting to MouseMotion/ButtonInput<MouseButton>
    // if grabbing is discarded?

    // TODO:: we use raw input avoiding PlayerCommand. Should probably fix:
    // For example if we'll ever add text input, pressing escape there will change focus
    // instead of discarding input
    if inputs.just_pressed(KeyCode::Escape) {
        *discard_grabbing = !*discard_grabbing;
    }

    let should_grab = window.focused && !*discard_grabbing;

    if should_grab {
        ensure_grabbed_cursor(&mut window);
    } else {
        ensure_released_cursor(&mut window);
    }
}

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerCommand>();
        app.add_systems(PreUpdate, grab_focused_window);
        app.add_systems(Update, spawn_camera.run_if(in_state(GameState::Init)));
        app.add_systems(
            PlayerInputPreUpdate,
            (
                player_look,
                player_keyboard_input,
                player_move_with_mouse_wheel,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            PlayerInputPostUpdate,
            (player_cmd_quit, player_cmd_move_camera).run_if(in_state(GameState::Game)),
        );
    }
}

#[cfg(test)]
#[path = "./tests/test_player_control_plugin.rs"]
mod test_player_control_plugin;
