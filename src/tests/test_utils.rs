#![cfg(test)]
#![allow(dead_code)]
use bevy::{input::keyboard::KeyboardInput, prelude::*};

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

pub fn contains_exact_event<E>(app: &App, event: E) -> bool
where
    E: Event + Eq,
{
    app.world()
        .resource::<Events<E>>()
        .iter_current_update_events()
        .any(|ev| *ev == event)
}
