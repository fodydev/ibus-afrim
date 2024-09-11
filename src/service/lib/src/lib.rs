#![allow(non_upper_case_globals)]
use std::path::Path;
use std::ffi::{CStr, c_char};


use ibus::*;

use log::{self};
use simple_log;
use simple_log::LogConfigBuilder;

mod afrim_api;

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

    Box::into_raw(Box::new(EngineCore {
        is_ctrl_released: true,
        is_idle: false,
        parent_engine: parent_engine,
        parent_engine_class: parent_engine_class,
    }))
}

impl EngineCore {
    pub unsafe fn from(ibus_afrim_engine: *mut IBusAfrimEngine) -> *mut Self {
        log::info!("getting the core engine...");

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
        "process key {:?} keychar={:?} keyval={} keycode={} modifiers={}",
        keyname,
        keychar,
        keyval,
        keycode,
        modifiers
    );

    let engine_core_ptr = EngineCore::from(engine as *mut IBusAfrimEngine);

    match (keyval, modifiers) {
        // Handling of the idle state.
        (
            IBUS_KEY_Control_L | IBUS_KEY_Control_R,
            IBusModifierType_IBUS_CONTROL_MASK,
        ) => {
            log::info!("toggle idle state...");

            (*engine_core_ptr).is_idle = !(*engine_core_ptr).is_idle;
            log::info!("idle state={}", (*engine_core_ptr).is_idle);
        }
        _ if (*engine_core_ptr).is_idle => (),
        (_, IBusModifierType_IBUS_CONTROL_MASK) => (),
        // Process other key events
        // We will manage the onpress event
        (_, 0) if keychar != '\0' => {
            // TODO
        }
        // Probably somthing that we don't want to manage
        _ => (),
    }

    /*
    let afrim_ptr = afrim_api::Singleton::get_afrim();
    if let Some(afrim) = (*afrim_ptr).as_mut() {
        //afrim.preprocessor.process(keyval);

        let input = afrim.preprocessor.get_input();
        log::info!("input: {}", input);
    } else {
        log::info!("Configuration of Afrim...");

        let afrim = afrim_api::Afrim::from_config(
            "/home/pythonbrad/Documents/Personal/Project/afrim-project/afrim-data/fmp/fmp.toml",
        );
        match afrim {
            Ok(afrim) => {
                afrim_api::Singleton::update_afrim(afrim);
                log::info!("Afrim configurated...");
            }
            Err(err) => log::error!("Configuration of Afrim failed: {err:?}"),
        }
    }
    */

    GBOOL_FALSE
}

#[no_mangle]
pub unsafe extern "C" fn configure_logging() {
    static DATA_DIRNAME: &str = "ibus-afrim";

    let log_dir = std::env::var("XDG_DATA_HOME")
        .map(|dir| Path::new(dir.as_str()).to_path_buf())
        .or(std::env::var("HOME").map(|home| Path::new(home.as_str()).join(".local").join("share")))
        .map(|path| path.join(DATA_DIRNAME).join("debug.log"))
        .unwrap_or(Path::new("/dev/null").to_path_buf());

    let config = LogConfigBuilder::builder()
        .path(log_dir.to_str().unwrap())
        .size(1 * 100)
        .roll_count(10)
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config).unwrap();

    log::info!("Logging initialized");
}
