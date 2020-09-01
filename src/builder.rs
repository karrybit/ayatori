use crate::model::{Resource, Service};
use petgraph::graph::{Graph, Node};

fn build(topics: Vec<Service>, subscriptions: Vec<Service>) {
    let mut graph = Graph::<Service, Resource>::new();
    let nodes = subscriptions
        .iter()
        .map(|service| graph.add_node(service.clone()))
        .collect::<Vec<_>>();
}
