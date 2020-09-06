use petgraph;
use rayon::prelude::*;
use std::io;

mod builder;
mod model;
mod parser;
mod scanner;

pub fn run(
    environment: String,
    base_file_path: String,
    topic_file_path: String,
    subscription_file_path: String,
) -> Result<petgraph::graph::Graph<String, String>, io::Error> {
    let (topic_files, topic_contents) =
        scanner::scan(&environment, &base_file_path, &topic_file_path)?;
    let (subscription_files, subscription_contents) =
        scanner::scan(&environment, &base_file_path, &subscription_file_path)?;

    let topic_services = parse_services(topic_files, topic_contents);
    let subscription_services = parse_services(subscription_files, subscription_contents);

    let graph = builder::build(topic_services, subscription_services);

    Ok(graph)
}

fn parse_services(files: Vec<String>, contents: Vec<String>) -> Vec<model::Service> {
    files
        .into_iter()
        .zip(contents.into_iter())
        .map(|(file, content)| {
            let splited = file.split('/').last();
            let file_name = splited.map_or("", |s| s.trim_matches('"')).to_owned();
            let resource = parser::parse(content);
            model::Service::new(file_name, resource)
        })
        .collect()
}

pub fn par_run(
    environment: String,
    base_file_path: String,
    topic_file_path: String,
    subscription_file_path: String,
) -> Result<petgraph::graph::Graph<String, String>, io::Error> {
    let (topic_files, topic_contents) =
        scanner::scan(&environment, &base_file_path, &topic_file_path)?;
    let (subscription_files, subscription_contents) =
        scanner::scan(&environment, &base_file_path, &subscription_file_path)?;

    let topic_parse_handle =
        std::thread::spawn(move || parse_par_services(topic_files, topic_contents));
    let subscription_parse_handle =
        std::thread::spawn(move || parse_par_services(subscription_files, subscription_contents));

    let topic_services = topic_parse_handle.join().unwrap();
    let subscription_services = subscription_parse_handle.join().unwrap();

    let graph = builder::par_build(topic_services, subscription_services);

    Ok(graph)
}

fn parse_par_services(files: Vec<String>, contents: Vec<String>) -> Vec<model::Service> {
    files
        .into_par_iter()
        .zip(contents.into_par_iter())
        .map(|(file, content)| {
            let splited = file.split('/').last();
            let file_name = splited.map_or("", |s| s.trim_matches('"')).to_owned();
            let resource = parser::parse(content);
            model::Service::new(file_name, resource)
        })
        .collect()
}
