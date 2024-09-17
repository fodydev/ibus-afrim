#![allow(non_upper_case_globals)]
use env_logger::{self};
use ibus::*;
use log::{self};
use std::ffi::CStr;
use std::os::raw::c_char;

mod afrim_api;
mod utils;

#[repr(C)]
/// Core structure of the IME.
pub struct EngineCore {
    is_idle: bool,
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
}

#[no_mangle]
/// Initialize the IME.
pub unsafe extern "C" fn new_engine_core(
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
) -> *mut EngineCore {
    log::info!("initializing the core engine...");
    let engine_core_ptr = Box::into_raw(Box::new(EngineCore {
        is_idle: false,
        parent_engine,
        parent_engine_class,
    }));

    log::info!("core engine initialized!");

    engine_core_ptr
}

impl EngineCore {
    pub unsafe fn from(ibus_afrim_engine: *mut IBusAfrimEngine) -> *mut Self {
        (*ibus_afrim_engine).engine_core as *mut Self
    }
}

#[no_mangle]
/// Release the memory used by the engine.
///
/// Note that, it won't be usable after this action.
pub unsafe extern "C" fn free_engine_core(engine_state: *mut EngineCore) {
    log::info!("releasing the memory...");
    std::mem::drop(Box::from_raw(engine_state));
    afrim_api::Singleton::drop_afrim();

    log::info!("memory released!")
}

#[no_mangle]
/// Selects the next predicates
pub unsafe extern "C" fn ibus_afrim_engine_page_down_button(engine: *mut IBusEngine) {
    log::info!("pagedown button!");
    let afrim_engine_core_ptr = engine as *mut IBusAfrimEngine;

    ibus_lookup_table_cursor_down((*afrim_engine_core_ptr).table);
    ibus_engine_update_lookup_table_fast(engine, (*afrim_engine_core_ptr).table, GBOOL_TRUE);
}

#[no_mangle]
/// Selects the previous predicates.
pub unsafe extern "C" fn ibus_afrim_engine_page_up_button(engine: *mut IBusEngine) {
    log::info!("pageup button!");
    let afrim_engine_core_ptr = engine as *mut IBusAfrimEngine;

    ibus_lookup_table_cursor_up((*afrim_engine_core_ptr).table);
    ibus_engine_update_lookup_table_fast(engine, (*afrim_engine_core_ptr).table, GBOOL_TRUE);
}

#[no_mangle]
/// Action to perform when the user is focus on a text field.
pub unsafe extern "C" fn ibus_afrim_engine_focus_in(_engine: *mut IBusEngine) {
    log::info!("focus in!");
}

#[no_mangle]
/// Action to perform when the user is not focus on a text field.
pub unsafe extern "C" fn ibus_afrim_engine_focus_out(_engine: *mut IBusEngine) {
    log::info!("focus out!");
}

#[no_mangle]
/// Enables the IME.
pub unsafe extern "C" fn ibus_afrim_engine_enable(engine: *mut IBusEngine) {
    log::info!("enabled!");
    // Request to use surrounding text feature
    ibus_engine_get_surrounding_text(
        engine,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
}

#[no_mangle]
/// Disables the IME.
pub unsafe extern "C" fn ibus_afrim_engine_disable(_engine: *mut IBusEngine) {
    log::info!("disabled!");
}

#[no_mangle]
/// Resets the engine to the initial state.
pub unsafe extern "C" fn ibus_afrim_engine_reset(engine: *mut IBusEngine) {
    log::info!("reset!");
    ibus_engine_hide_auxiliary_text(engine);
    ibus_engine_hide_lookup_table(engine);

    let afrim_ptr = afrim_api::Singleton::get_afrim();
    if let Some(afrim) = (*afrim_ptr).as_mut() {
        afrim.preprocessor.process(Default::default());
    }
}

#[no_mangle]
/// Process key events.
pub unsafe extern "C" fn ibus_afrim_engine_process_key_event(
    engine: *mut IBusEngine,
    keyval: guint,
    keycode: guint,
    modifiers: guint,
) -> gboolean {
    let keyname = CStr::from_ptr(ibus_keyval_name(keyval) as *const c_char);
    let keychar = char::from_u32_unchecked(ibus_keyval_to_unicode(keyval));
    log::info!(
        "processing key={:?} keychar={:?} keyval={:?} keycode={:?} modifiers={:?}...",
        keyname,
        keychar,
        keyval,
        keycode,
        modifiers
    );

    let afrim_engine_core_ptr = engine as *mut IBusAfrimEngine;
    let engine_core_ptr = EngineCore::from(afrim_engine_core_ptr);
    let afrim_ptr = afrim_api::Singleton::get_afrim();

    match (keyval, modifiers) {
        // Handling of the idle state.
        (IBUS_KEY_Control_L | IBUS_KEY_Control_R, IBusModifierType_IBUS_CONTROL_MASK) => {
            log::info!("toggle idle state...");

            (*engine_core_ptr).is_idle = !(*engine_core_ptr).is_idle;
            log::info!("idle state={:?}", (*engine_core_ptr).is_idle);
        }
        _ if (*engine_core_ptr).is_idle => (),
        // Handling special functions
        (IBUS_KEY_Shift_L, IBusModifierType_IBUS_CONTROL_MASK) => {
            ibus_afrim_engine_page_up_button(engine)
        }
        (IBUS_KEY_Shift_R, IBusModifierType_IBUS_CONTROL_MASK) => {
            ibus_afrim_engine_page_down_button(engine)
        }
        (IBUS_KEY_space, IBusModifierType_IBUS_CONTROL_MASK) => {
            let index = ibus_lookup_table_get_cursor_pos((*afrim_engine_core_ptr).table);
            let ibus_selected_label =
                ibus_lookup_table_get_candidate((*afrim_engine_core_ptr).table, index);
            let selected_label_ptr = (*ibus_selected_label).text;
            let selected_candidate = CStr::from_ptr(selected_label_ptr).to_str().unwrap();

            if let Some(afrim) = (*afrim_ptr).as_mut() {
                log::info!(
                    "send preprocessor command commit_text={:?}",
                    selected_candidate
                );
                afrim.preprocessor.commit(selected_candidate.to_string());
            }
        }
        // Maybe the user is doing another thing.
        (_, IBusModifierType_IBUS_CONTROL_MASK) => ibus_afrim_engine_reset(engine),
        // These keys should be ignored at this point
        (IBUS_KEY_Control_L | IBUS_KEY_Control_R | IBUS_KEY_Shift_L | IBUS_KEY_Shift_R, 0) => (),
        // We leave `afrim-preprocessor` handles key press events
        _ if modifiers | IBusModifierType_IBUS_RELEASE_MASK != modifiers => {
            if let Some(afrim) = (*afrim_ptr).as_mut() {
                let event = utils::ibus_keypress_event_to_afrim_key_event(keyval);
                afrim.preprocessor.process(event);

                let input = afrim.preprocessor.get_input();
                log::info!("afrim buffer_text={:?}", &input);
                let ibus_text = utils::string_to_ibus_text(input.to_string());
                ibus_engine_update_auxiliary_text(engine, ibus_text, GBOOL_FALSE);

                // Refresh the candidate list
                ibus_lookup_table_clear((*afrim_engine_core_ptr).table);

                let mut index = 0;
                for predicate in afrim.translator.translate(&input) {
                    for text in predicate.texts {
                        if text.is_empty() {
                            continue;
                        };

                        let ibus_label_text =
                            utils::string_to_ibus_text(format!("~{}", predicate.remaining_code));
                        ibus_lookup_table_set_label(
                            (*afrim_engine_core_ptr).table,
                            index,
                            ibus_label_text,
                        );

                        let ibus_text = utils::string_to_ibus_text(text);
                        ibus_lookup_table_append_candidate(
                            (*afrim_engine_core_ptr).table,
                            ibus_text,
                        );

                        index += 1;
                    }
                }

                if index > 0 {
                    ibus_engine_show_auxiliary_text(engine);
                    ibus_engine_update_lookup_table_fast(
                        engine,
                        (*afrim_engine_core_ptr).table,
                        GBOOL_TRUE,
                    );
                } else {
                    ibus_engine_hide_auxiliary_text(engine);
                    ibus_engine_hide_lookup_table(engine);
                }
            }
        }
        // Process `afrim-preprocessor` instructions on release
        _ => {
            while let Some(command) = (*afrim_ptr)
                .as_mut()
                .and_then(|afrim| afrim.preprocessor.pop_queue())
            {
                log::info!("executing command={:?}...", &command);
                match command {
                    afrim_api::Command::CommitText(text) => {
                        let ibus_text = utils::string_to_ibus_text(text);
                        ibus_engine_commit_text(engine, ibus_text);
                    }
                    afrim_api::Command::CleanDelete => {}
                    afrim_api::Command::Delete => {
                        ibus_engine_delete_surrounding_text(engine, -1, 1);
                        // Some applications require this delay to work properly
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    afrim_api::Command::Pause => {
                        (*engine_core_ptr).is_idle = true;
                    }
                    afrim_api::Command::Resume => {
                        (*engine_core_ptr).is_idle = false;
                    }
                };
                log::info!("command executed!");
            }
        }
    }

    log::info!("key processed!");
    GBOOL_FALSE
}

#[no_mangle]
/// Loads the afrim configuration.
pub unsafe extern "C" fn configure_afrim() {
    let afrim_ptr = afrim_api::Singleton::get_afrim();
    if (*afrim_ptr).is_none() {
        log::info!("configuration of Afrim...");

        let afrim = afrim_api::Afrim::from_config(
            "/home/pythonbrad/Documents/Personal/Project/afrim-project/afrim-data/am/am.toml",
        );
        match afrim {
            Ok(afrim) => {
                afrim_api::Singleton::update_afrim(afrim);
                log::info!("afrim configurated!");
            }
            Err(err) => log::error!("configuration of Afrim failed: {err:?}"),
        }
    }
}

#[no_mangle]
/// Intitializes the logging.
pub extern "C" fn configure_logging() {
    env_logger::init();
}
