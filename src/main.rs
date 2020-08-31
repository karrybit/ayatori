use clap::{App, Arg};
use std::io;

mod lexer;
mod parser;
mod resource;
mod scanner;
mod subscription;
mod token_type;

fn main() -> Result<(), io::Error> {
    let matches = App::new("ayatori")
        .version("0.1.0")
        .author("Takumi Karibe <takumi.k.5610@gmail.com>")
        .about("Analysis of dependency between services in microservices")
        .arg(
            Arg::with_name("environment")
                .help("environment")
                .takes_value(true)
                .short("e")
                .long("environment")
                .possible_values(&["develop", "staging", "production"])
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .help("base file path")
                .takes_value(true)
                .short("b")
                .long("base_path")
                .required(true),
        )
        .arg(
            Arg::with_name("subscriber")
                .help("Subscription files name")
                .takes_value(true)
                .short("s")
                .long("subscriber")
                .required(true),
        )
        .arg(
            Arg::with_name("publisher")
                .help("Publisher files name")
                .takes_value(true)
                .short("p")
                .long("publisher")
                .required(true),
        )
        .get_matches();

    let environment = matches
        .value_of("environment")
        .unwrap_or_else(|| panic!("environment is required"));
    let base_path = matches
        .value_of("path")
        .unwrap_or_else(|| panic!("path file path is required"));
    let subscriber_file_name = matches
        .value_of("subscriber")
        .unwrap_or_else(|| panic!("subscriber file name is required"));
    let publisher_file_name = matches
        .value_of("publisher")
        .unwrap_or_else(|| panic!("publisher file name is required"));

    let (subscription_files, subscription_contents) =
        scanner::scan(&environment, &base_path, &subscriber_file_name)?;
    let subscriptions = parse_subscription(subscription_files, subscription_contents);
    Ok(())
}

fn parse(content: String) -> Vec<resource::Resource> {
    if content.is_empty() {
        return vec![];
    }
    let mut lexer = lexer::Lexer::new(content);
    let mut parser = parser::Parser::new(&mut lexer);
    let resources = parser.parse_resources();
    resources
}

fn parse_subscription(
    files: Vec<String>,
    contents: Vec<String>,
) -> Vec<subscription::Subscription> {
    let service_names = files
        .into_iter()
        .map(|file| {
            let splited = file.split('/').last();
            splited.map_or("", |s| s.trim_matches('"')).to_owned()
        })
        .collect::<Vec<String>>();
    let resources = contents
        .into_iter()
        .map(|content| parse(content))
        .collect::<Vec<Vec<resource::Resource>>>();
    service_names
        .into_iter()
        .zip(resources.into_iter())
        .map(|(service_name, resource)| subscription::Subscription::new(service_name, resource))
        .collect::<Vec<subscription::Subscription>>()
}
