use crate::model::{Service, ValueContainer, ValueType};
use petgraph::graph::Graph;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub(crate) fn build(topics: Vec<Service>, subscriptions: Vec<Service>) -> Graph<String, String> {
    let mut graph = Graph::<String, String>::new();
    let nodes = subscriptions
        .iter()
        .map(|service| graph.add_node(service.name.clone()))
        .collect::<Vec<_>>();

    subscriptions
        .iter()
        .enumerate()
        .for_each(|(i, subscription)| {
            subscription.resources.iter().for_each(|resource| {
                let subscribing_topic = match resource.attributes.get("topic_arn") {
                    Some(ValueContainer::Value(ValueType::Str(subscribing_topic))) => {
                        subscribing_topic
                    }
                    _ => panic!(),
                };
                topics.iter().for_each(|topic| {
                    topic.resources.iter().for_each(|topic_resource| {
                        let policy = match topic_resource.attributes.get("policy") {
                            Some(ValueContainer::Dictionary(policy)) => policy,
                            _ => panic!(),
                        };
                        let statements = match policy.get("Statement") {
                            Some(ValueContainer::Array(statements)) => statements,
                            _ => panic!(),
                        };
                        let statement = match statements.get(0) {
                            Some(statement) => statement,
                            _ => panic!(),
                        };
                        let statement = match statement.as_ref() {
                            ValueContainer::Dictionary(statement) => statement,
                            _ => panic!(),
                        };
                        let resource = match statement.get("Resource") {
                            Some(ValueContainer::Value(ValueType::Str(resource))) => resource,
                            _ => panic!(),
                        };
                        if subscribing_topic == resource {
                            if let Some(j) = subscriptions
                                .iter()
                                .position(|subscription| subscription.name == topic.name)
                            {
                                graph.add_edge(nodes[j], nodes[i], resource.clone());
                            }
                        };
                    });
                });
            });
        });

    graph
}

pub(crate) fn par_build(
    topics: Vec<Service>,
    subscriptions: Vec<Service>,
) -> Graph<String, String> {
    let graph = Arc::new(Mutex::new(Graph::<String, String>::new()));
    let nodes = subscriptions
        .par_iter()
        .map(|service| graph.lock().unwrap().add_node(service.name.clone()))
        .collect::<Vec<_>>();

    subscriptions
        .par_iter()
        .enumerate()
        .for_each(|(i, subscription)| {
            subscription.resources.par_iter().for_each(|resource| {
                let subscribing_topic = match resource.attributes.get("topic_arn") {
                    Some(ValueContainer::Value(ValueType::Str(subscribing_topic))) => {
                        subscribing_topic
                    }
                    _ => panic!(),
                };
                topics.par_iter().for_each(|topic| {
                    topic.resources.par_iter().for_each(|topic_resource| {
                        let policy = match topic_resource.attributes.get("policy") {
                            Some(ValueContainer::Dictionary(policy)) => policy,
                            _ => panic!(),
                        };
                        let statements = match policy.get("Statement") {
                            Some(ValueContainer::Array(statements)) => statements,
                            _ => panic!(),
                        };
                        let statement = match statements.get(0) {
                            Some(statement) => statement,
                            _ => panic!(),
                        };
                        let statement = match statement.as_ref() {
                            ValueContainer::Dictionary(statement) => statement,
                            _ => panic!(),
                        };
                        let resource = match statement.get("Resource") {
                            Some(ValueContainer::Value(ValueType::Str(resource))) => resource,
                            _ => panic!(),
                        };
                        if subscribing_topic == resource {
                            if let Some(j) = subscriptions
                                .iter()
                                .position(|subscription| subscription.name == topic.name)
                            {
                                graph.lock().unwrap().add_edge(
                                    nodes[j],
                                    nodes[i],
                                    resource.clone(),
                                );
                            }
                        };
                    });
                });
            });
        });

    Arc::try_unwrap(graph).unwrap().into_inner().unwrap()
}
