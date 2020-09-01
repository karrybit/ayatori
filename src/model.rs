use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) enum ResourceType {
    Topic,
    Subscription,
}

impl ResourceType {
    pub(crate) fn from_str(str: &str) -> Self {
        match str {
            "aws_sns_topic" => ResourceType::Topic,
            "aws_sns_topic_subscription" => ResourceType::Subscription,
            _ => panic!("invalid resource type from str"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Resource {
    resource_type: ResourceType,
    event_name: String,
    attributes: HashMap<String, String>,
}

impl Resource {
    pub(crate) fn new(
        resource_type: ResourceType,
        event_name: String,
        attributes: HashMap<String, String>,
    ) -> Self {
        Resource {
            resource_type,
            event_name,
            attributes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Service {
    name: String,
    resources: Vec<Resource>,
}

impl Service {
    pub(crate) fn new(name: String, resources: Vec<Resource>) -> Self {
        Self { name, resources }
    }
}
