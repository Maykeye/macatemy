use bevy::prelude::*;

#[cfg(feature = "light-control")]
#[derive(Component, Reflect, Default)]
pub struct LightControl {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub update: bool,
}

fn spawn_light(mut commands: Commands) {
    let pitch = 106.5f32.to_radians();
    let yaw = 106.5f32.to_radians();
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, pitch, yaw, 0.0)),
        #[cfg(feature = "light-control")]
        LightControl::default(),
    ));
}

#[cfg(feature = "light-control")]
fn update_light(q: Query<(&mut LightControl, &mut Transform)>) {
    for (light_ctrl, mut tr) in q {
        let pitch = light_ctrl.pitch.to_radians();
        let yaw = light_ctrl.yaw.to_radians();
        let roll = light_ctrl.roll.to_radians();
        tr.rotation = Quat::from_euler(EulerRot::XYZ, pitch, yaw, roll);
    }
}

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);

        #[cfg(feature = "light-control")]
        {
            app.add_systems(Update, update_light);
            app.register_type::<LightControl>();
        }
    }
}
