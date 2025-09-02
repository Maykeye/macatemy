use bevy::prelude::*;

fn spawn_light(mut commands: Commands) {
    commands.spawn((DirectionalLight::default(),));
}

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}
