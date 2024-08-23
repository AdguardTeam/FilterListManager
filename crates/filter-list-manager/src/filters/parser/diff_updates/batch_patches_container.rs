use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Container for batch patch files.
/// We should download each file once and share between few filters
pub(crate) struct BatchPatchesContainer {
    map: HashMap<String, String>,
}

impl BatchPatchesContainer {
    pub(crate) fn factory() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            map: HashMap::new(),
        }))
    }

    /// Gets an own copy of patch file contents by url
    pub(crate) fn get_a_copy(&self, absolute_url: &String) -> Option<String> {
        self.map.get(absolute_url).map(ToOwned::to_owned)
    }

    /// Inserts patch body by absolute url
    pub(crate) fn insert(&mut self, absolute_url: String, body: String) {
        self.map.insert(absolute_url, body);
    }
}
