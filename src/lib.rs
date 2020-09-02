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
) -> Result<(Vec<model::Service>, Vec<model::Service>), io::Error> {
    let (topic_files, topic_contents) =
        scanner::scan(&environment, &base_file_path, &topic_file_path)?;
    let (subscription_files, subscription_contents) =
        scanner::scan(&environment, &base_file_path, &subscription_file_path)?;
    let topic_services = parse_services(topic_files, topic_contents);
    let subscription_services = parse_services(subscription_files, subscription_contents);
    Ok((topic_services, subscription_services))
}

fn parse_services(files: Vec<String>, contents: Vec<String>) -> Vec<model::Service> {
    let service_names = files
        .into_iter()
        .map(|file| {
            let splited = file.split('/').last();
            splited.map_or("", |s| s.trim_matches('"')).to_owned()
        })
        .collect::<Vec<String>>();
    let resources = contents
        .into_iter()
        .map(|content| parser::parse(content))
        .collect::<Vec<Vec<model::Resource>>>();
    service_names
        .into_iter()
        .zip(resources.into_iter())
        .map(|(name, resources)| model::Service::new(name, resources))
        .collect::<Vec<model::Service>>()
}
