use crate::resource::Resource;

#[derive(Debug)]
pub(crate) struct Service {
    name: String,
    resources: Vec<Resource>,
}

impl Service {
    pub(crate) fn new(name: String, resources: Vec<Resource>) -> Self {
        Self { name, resources }
    }
}
