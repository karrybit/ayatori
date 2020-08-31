use crate::resource;

#[derive(Debug)]
pub(crate) struct Subscription {
    service_name: String,
    dependencies: Vec<resource::Resource>,
}

impl Subscription {
    pub(crate) fn new(service_name: String, dependencies: Vec<resource::Resource>) -> Self {
        Self {
            service_name,
            dependencies,
        }
    }
}
