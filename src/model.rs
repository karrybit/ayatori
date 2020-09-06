use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) enum ResourceKind {
    Topic,
    Subscription,
}

impl ResourceKind {
    pub(crate) fn from_str(str: &str) -> Self {
        match str {
            "aws_sns_topic" => ResourceKind::Topic,
            "aws_sns_topic_subscription" => ResourceKind::Subscription,
            _ => panic!("invalid resource type from str"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ValueType {
    Str(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub(crate) enum ValueContainer {
    Dictionary(HashMap<String, ValueContainer>),
    Array(Vec<Box<ValueContainer>>),
    Value(ValueType),
}

#[derive(Debug, Clone)]
pub(crate) struct Resource {
    kind: ResourceKind,
    name: String,
    pub(crate) attributes: HashMap<String, ValueContainer>,
}

impl Resource {
    pub(crate) fn new(
        kind: ResourceKind,
        name: String,
        attributes: HashMap<String, ValueContainer>,
    ) -> Self {
        Resource {
            kind,
            name,
            attributes,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Service {
    pub(crate) name: String,
    pub(crate) resources: Vec<Resource>,
}

impl Service {
    pub(crate) fn new(name: String, resources: Vec<Resource>) -> Self {
        Self { name, resources }
    }
}
