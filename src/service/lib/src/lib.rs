#![allow(non_upper_case_globals)]
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use ibus::*;

use env_logger::{self};
use log::{self};

mod afrim_api;

mod utils;

#[repr(C)]
pub struct EngineCore {
    is_ctrl_released: bool,
    is_idle: bool,
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
}

#[no_mangle]
pub unsafe extern "C" fn new_engine_core(
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
) -> *mut EngineCore {
    log::info!("initializing the core engine...");
    let engine_core_ptr = Box::into_raw(Box::new(EngineCore {
        is_ctrl_released: true,
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
pub unsafe extern "C" fn free_engine_core(engine_state: *mut EngineCore) {
    log::info!("releasing the memory...");
    std::mem::drop(Box::from_raw(engine_state));
    afrim_api::Singleton::drop_afrim();
    log::info!("memory released!")
}

#[no_mangle]
pub unsafe extern "C" fn ibus_afrim_engine_page_down_button(_engine: *mut IBusEngine) {}

#[no_mangle]
pub unsafe extern "C" fn ibus_afrim_engine_page_up_button(_engine: *mut IBusEngine) {}

#[no_mangle]
pub unsafe extern "C" fn ibus_afrim_engine_focus_out(_engine: *mut IBusEngine) {}

#[no_mangle]
pub unsafe extern "C" fn ibus_afrim_engine_candidate_clicked(
    _engine: *mut IBusEngine,
    _indx: guint,
    _button_state: guint,
    _keyboard_state: guint,
) {
}

#[no_mangle]
pub unsafe extern "C" fn ibus_afrim_engine_process_key_event(
    engine: *mut IBusEngine,
    keyval: guint,
    keycode: guint,
    modifiers: guint,
) -> gboolean {
    let keyname = CStr::from_ptr(ibus_keyval_name(keyval) as *const c_char);
    let keychar = char::from_u32_unchecked(ibus_keyval_to_unicode(keyval));
    log::info!(
        "processing key={:?} keychar={:?} keyval={} keycode={} modifiers={}...",
        keyname,
        keychar,
        keyval,
        keycode,
        modifiers
    );

    let engine_core_ptr = EngineCore::from(engine as *mut IBusAfrimEngine);
    let afrim_ptr = afrim_api::Singleton::get_afrim();

    match (keyval, modifiers) {
        // Handling of the idle state.
        (IBUS_KEY_Control_L | IBUS_KEY_Control_R, IBusModifierType_IBUS_CONTROL_MASK) => {
            log::info!("toggle idle state...");

            (*engine_core_ptr).is_idle = !(*engine_core_ptr).is_idle;
            log::info!("idle state={}", (*engine_core_ptr).is_idle);
        }
        _ if (*engine_core_ptr).is_idle => (),
        // These keys should be ignored at this point
        (IBUS_KEY_Control_L | IBUS_KEY_Control_R, _) => (),
        // We leave `afrim-preprocessor` handles key press events
        _ if modifiers | IBusModifierType_IBUS_RELEASE_MASK != modifiers => {
            let event = utils::ibus_keypress_event_to_afrim_key_event(keyval);
            if let Some(afrim) = (*afrim_ptr).as_mut() {
                afrim.preprocessor.process(event);
                log::info!("afrim buffer_text={}", afrim.preprocessor.get_input());

                // TODO: refresh the translator
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
                        let text_ptr = CString::new(text).unwrap().into_raw();
                        let ibus_text = ibus_text_new_from_string(text_ptr as *const gchar);
                        ibus_engine_commit_text(engine, ibus_text);

                        drop(CString::from_raw(text_ptr));
                    }
                    afrim_api::Command::CleanDelete => {}
                    afrim_api::Command::Delete => {
                        ibus_engine_delete_surrounding_text(engine, -1, 1);
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

    let afrim_ptr = afrim_api::Singleton::get_afrim();
    if (*afrim_ptr).is_none() {
        log::info!("configuration of Afrim...");

        let afrim = afrim_api::Afrim::from_config(
            "/home/pythonbrad/Documents/Personal/Project/afrim-project/afrim-data/fmp/fmp.toml",
        );
        match afrim {
            Ok(afrim) => {
                afrim_api::Singleton::update_afrim(afrim);
                log::info!("afrim configurated!");
            }
            Err(err) => log::error!("configuration of Afrim failed: {err:?}"),
        }
    }

    log::info!("key processed!");
    GBOOL_FALSE
}

#[no_mangle]
pub extern "C" fn configure_logging() {
    env_logger::init();
}
