use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Resource {
    event_name: String,
    attributes: HashMap<String, String>,
}

impl Resource {
    pub(crate) fn new(event_name: String, attributes: HashMap<String, String>) -> Self {
        Resource {
            event_name,
            attributes,
        }
    }
}
