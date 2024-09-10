#![deny(missing_docs)]
/// Binding of the afrim api.
///
use afrim_config::Config;
use afrim_preprocessor::Preprocessor;
use afrim_translator::Translator;
use anyhow::Result;
use std::path::Path;
use std::sync::LazyLock;

pub struct Afrim {
    pub preprocessor: Preprocessor,
    pub translator: Translator,
}

impl Afrim {
    /// Initializes an Afrim instance based on the provided configuration file.
    pub fn from_config(config_file: &str) -> Result<Self> {
        let config_file = Path::new(&config_file);
        let config = Config::from_file(config_file)?;

        // Core
        let auto_commit = config
            .core
            .as_ref()
            .and_then(|c| c.auto_commit)
            .unwrap_or(false);
        let buffer_size = config
            .core
            .as_ref()
            .and_then(|c| c.buffer_size)
            .unwrap_or(64);

        // Data
        let data = config.extract_data();
        let data = data
            .iter()
            .map(|(key, value)| vec![key.as_str(), value.as_str()])
            .collect();
        let map = utils::build_map(data);
        let preprocessor = Preprocessor::new(map.into(), buffer_size);

        // Translation
        let translation = config.extract_translation();
        #[cfg(feature = "rhai")]
        let mut translator = Translator::new(translation, auto_commit);
        #[cfg(not(feature = "rhai"))]
        let translator = Translator::new(translation, auto_commit);

        // Translators
        #[cfg(feature = "rhai")]
        config
            .extract_translators()?
            .into_iter()
            .for_each(|(name, ast)| {
                translator.register(name, ast);
            });

        Ok(Self {
            preprocessor,
            translator,
        })
    }
}

/// A pointer to an afrim instance.
pub struct Singleton;

impl Singleton {
    /// Initializes a pointer to an afrim instance.
    ///
    /// Note that the resulting singletion is not thread safe.
    pub fn init_afrim() -> usize {
        static CURRENT: LazyLock<usize> = LazyLock::new(|| {
            let instance_ptr = Box::into_raw(Box::new(None::<Afrim>));

            instance_ptr as usize
        });

        *CURRENT
    }

    /// Returns the current afrim instance pointer.
    pub fn get_afrim() -> *mut Option<Afrim> {
        Self::init_afrim() as *mut Option<Afrim>
    }

    /// Updates the current afrim instance.
    pub fn update_afrim(afrim: Afrim) {
        let instance_ptr = Self::get_afrim();

        unsafe {
            *instance_ptr = Some(afrim);
        }
    }

    /// Drop the current afrim instance.
    ///
    /// Note that this action will free the memory, and is irreversible.
    pub unsafe fn drop_afrim() {
        let instance_ptr = Self::get_afrim();

        drop(Box::from_raw(instance_ptr));
    }
}

mod utils {
    pub use afrim_preprocessor::utils::*;
}
