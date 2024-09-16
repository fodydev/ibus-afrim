use afrim_translator::Predicate;
use std::sync::LazyLock;

/// A pointer to an Predicator instance.
pub struct Predicator {
    predicates: Vec<Predicate>,
}

impl Predicator {
    /// Initializes a pointer to an predicator instance.
    ///
    /// Note that the resulting singletion is not thread safe.
    fn init() -> usize {
        static CURRENT: LazyLock<usize> = LazyLock::new(|| {
            let instance_ptr: *mut Predicator =
                Box::into_raw(Box::new(Predicator { predicates: vec![] }));

            instance_ptr as usize
        });

        *CURRENT
    }

    /// Returns the current predicator.
    pub fn get() -> *mut Predicator {
        Self::init() as *mut Predicator
    }

    /// Adds a predicate in the current Predicator instance.
    pub unsafe fn add_predicate(predicate: Predicate) {
        let instance_ptr = Self::get();

        predicate
            .texts
            .iter()
            .filter(|text| text.is_empty())
            .for_each(|text| {
                let mut predicate = predicate.clone();
                predicate.texts = vec![text.to_owned()];

                (*instance_ptr).predicates.push(predicate);
            });
    }

    /// Drop the current afrim instance.
    ///
    /// Note that this action will free the memory, and is irreversible.
    pub unsafe fn drop() {
        let instance_ptr = Self::get();

        drop(Box::from_raw(instance_ptr));
    }
}
