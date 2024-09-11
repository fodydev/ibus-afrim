use afrim_preprocessor::{Key, KeyState, KeyboardEvent};

/// Converts an IBusKeyboardEvent into a KeyboardEvent.
pub fn char_to_afrim_key_event(c: char) -> KeyboardEvent {
    let key = Key::Character(c.to_string());
    let state = KeyState::Down;

    KeyboardEvent {
        key,
        state,
        ..Default::default()
    }
}
