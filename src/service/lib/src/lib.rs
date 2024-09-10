#![allow(non_upper_case_globals)]
use std::path::Path;

use ibus::{gboolean, guint, IBusAfrimEngine, IBusEngine, IBusEngineClass, GBOOL_FALSE};

use log::{self};
use simple_log;
use simple_log::LogConfigBuilder;

#[derive(PartialEq)]
#[repr(C)]
enum InputMode {
    Normal,
    IME,
}

#[repr(C)]
pub struct EngineCore {
    input_mode: InputMode,
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
}

#[no_mangle]
pub unsafe extern "C" fn new_afrim_engine_core(
    parent_engine: *mut IBusAfrimEngine,
    parent_engine_class: *mut IBusEngineClass,
) -> *mut EngineCore {
    Box::into_raw(Box::new(EngineCore {
        input_mode: InputMode::Normal,
        parent_engine: parent_engine,
        parent_engine_class: parent_engine_class,
    }))
}

impl EngineCore {}

#[no_mangle]
pub unsafe extern "C" fn free_afrim_engine_core(engine_state: *mut EngineCore) {
    std::mem::drop(Box::from_raw(engine_state));
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
    _engine: *mut IBusEngine,
    keyval: guint,
    _keycode: guint,
    _modifiers: guint,
) -> gboolean {
    log::info!("{}", keyval as u8 as char);

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
