use clap;
use std::io;

mod lexer;
mod parser;
mod resource;
mod scanner;
mod subscription;
mod token_type;

struct Arg {
    environment: String,
    base_file_path: String,
    publisher_file_name: String,
    subscriber_file_name: String,
}

fn parse_arg() -> Arg {
    let matches = clap::App::new("ayatori")
        .version("0.1.0")
        .author("Takumi Karibe <takumi.k.5610@gmail.com>")
        .about("Analysis of dependency between services in microservices")
        .arg(
            clap::Arg::with_name("environment")
                .help("Environment")
                .takes_value(true)
                .short("e")
                .long("environment")
                .possible_values(&["develop", "staging", "production"])
                .required(true),
        )
        .arg(
            clap::Arg::with_name("base_file_path")
                .help("Base file path")
                .takes_value(true)
                .short("b")
                .long("base_file_path")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("publisher_file_name")
                .help("Publisher file name")
                .takes_value(true)
                .short("p")
                .long("publisher")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("subscriber_file_name")
                .help("Subscriber file name")
                .takes_value(true)
                .short("s")
                .long("subscriber")
                .required(true),
        )
        .get_matches();

    let environment = matches
        .value_of("environment")
        .unwrap_or_else(|| panic!("environment is required"));
    let base_file_path = matches
        .value_of("base_file_path")
        .unwrap_or_else(|| panic!("base path file path is required"));
    let publisher_file_name = matches
        .value_of("publisher_file_name")
        .unwrap_or_else(|| panic!("publisher file name is required"));
    let subscriber_file_name = matches
        .value_of("subscriber_file_name")
        .unwrap_or_else(|| panic!("subscriber file name is required"));

    Arg {
        environment: environment.to_owned(),
        base_file_path: base_file_path.to_owned(),
        publisher_file_name: publisher_file_name.to_owned(),
        subscriber_file_name: subscriber_file_name.to_owned(),
    }
}

fn main() -> Result<(), io::Error> {
    let arg = parse_arg();
    let (publish_files, subscription_contents) = scanner::scan(
        &arg.environment,
        &arg.base_file_path,
        &arg.publisher_file_name,
    )?;
    let (subscription_files, subscription_contents) = scanner::scan(
        &arg.environment,
        &arg.base_file_path,
        &arg.subscriber_file_name,
    )?;
    let subscriptions = parse_subscription(subscription_files, subscription_contents);
    dbg!(subscriptions);
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
