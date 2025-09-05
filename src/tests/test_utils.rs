#![cfg(test)]
#![allow(dead_code)]
use bevy::{
    app::PluginGroupBuilder,
    input::keyboard::KeyboardInput,
    prelude::*,
    render::{RenderPlugin, settings::WgpuSettings},
    winit::{WakeUp, WinitPlugin},
};

pub fn is_key_just_pressed(app: &App, keycode: KeyCode) -> bool {
    let input = app.world().resource::<ButtonInput<KeyCode>>();
    input.just_pressed(keycode)
}

pub fn keyboard_input_from_keycode(keycode: KeyCode, is_pressed: bool) -> KeyboardInput {
    use bevy::input::keyboard::Key;
    KeyboardInput {
        key_code: keycode,
        logical_key: Key::Unidentified(bevy::input::keyboard::NativeKey::Unidentified),
        state: if is_pressed {
            bevy::input::ButtonState::Pressed
        } else {
            bevy::input::ButtonState::Released
        },
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    }
}

pub fn press_key(app: &mut App, keycode: KeyCode) {
    let ev = keyboard_input_from_keycode(keycode, true);
    app.world_mut().send_event(ev);
}

pub fn release_key(app: &mut App, keycode: KeyCode) {
    let ev = keyboard_input_from_keycode(keycode, false);
    app.world_mut().send_event(ev);
}

pub fn contains_exact_event<E>(app: &App, event: E) -> bool
where
    E: Event + Eq,
{
    app.world()
        .resource::<Events<E>>()
        .iter_current_update_events()
        .any(|ev| *ev == event)
}

pub fn make_defaullt_plugins_for_headless_test() -> PluginGroupBuilder {
    // #[tests] are run in separate threads which winit doesn't like
    let mut winit = WinitPlugin::<WakeUp>::default();
    winit.run_on_any_thread = true;

    DefaultPlugins
        .set(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        })
        .set(winit)
}

/// Test if the entity is still alive
pub fn is_entity_alive(app: &App, ent: Entity) -> bool {
    app.world().get_entity(ent).is_ok()
}

pub fn get_resource<R: Resource>(app: &App) -> &R {
    app.world().get_resource::<R>().unwrap()
}

/// Calculates the max delta between any channel and average value of the channels.
/// Basically if the value is small, it's grayscale (white/gray/black)
pub fn rgb_max_avg_delta(color: LinearRgba) -> f32 {
    let avg = (color.red + color.green + color.blue) / 3.0;
    (color.red - avg)
        .abs()
        .min((color.green - avg).abs())
        .min((color.blue - avg).abs())
}

pub fn get_position(app: &App, ent: Entity) -> Vec3 {
    app.world().get::<Transform>(ent).unwrap().translation
}

pub trait BaseTestSuite {
    fn app(&mut self) -> &mut App;
    fn update(mut self) -> Self
    where
        Self: Sized,
    {
        self.app().update();
        self
    }
    fn press(mut self, keycode: KeyCode) -> Self
    where
        Self: Sized,
    {
        press_key(self.app(), keycode);
        self.update()
    }
    fn release(mut self, keycode: KeyCode) -> Self
    where
        Self: Sized,
    {
        release_key(&mut self.app(), keycode);
        self.update()
    }
}
