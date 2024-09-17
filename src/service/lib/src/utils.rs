use afrim_preprocessor::{Key, KeyState, KeyboardEvent};
use ibus::*;
use std::ffi::CString;

/// Converts an IBusKeyboardEvent into a KeyboardEvent.
pub unsafe fn ibus_keypress_event_to_afrim_key_event(keyval: guint) -> KeyboardEvent {
    let key = match keyval {
        IBUS_KEY_BackSpace => Key::Backspace,
        IBUS_KEY_Caps_Lock => Key::CapsLock,
        IBUS_KEY_Shift_L => Key::Shift,
        IBUS_KEY_Shift_R => Key::Shift,
        _ => char::from_u32(ibus_keyval_to_unicode(keyval))
            .filter(|c| c.is_alphanumeric() || c.is_ascii_punctuation())
            .map(|c| Key::Character(c.to_string()))
            .unwrap_or_default(),
    };

    KeyboardEvent {
        key,
        state: KeyState::Down,
        ..Default::default()
    }
}

/// Converts a string to ibus text.
pub unsafe fn string_to_ibus_text(text: String) -> *mut IBusText {
    let text_ptr = CString::new(text).unwrap().into_raw();
    let ibus_text = ibus_text_new_from_string(text_ptr);
    drop(CString::from_raw(text_ptr));

    ibus_text
}
