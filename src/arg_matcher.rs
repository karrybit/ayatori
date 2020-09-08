use clap;

pub(crate) struct Arg {
    pub(crate) environment: String,
    pub(crate) base_file_path: String,
    pub(crate) topic_file_name: String,
    pub(crate) subscription_file_name: String,
    pub(crate) output_format: String,
}

pub(crate) fn parse_arg() -> Arg {
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
            clap::Arg::with_name("topic_file_name")
                .help("Topic file name")
                .takes_value(true)
                .short("t")
                .long("topic")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("subscription_file_name")
                .help("Subscription file name")
                .takes_value(true)
                .short("s")
                .long("subscription")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output_format")
                .help("Output format")
                .short("f")
                .long("format")
                .possible_values(&["json", "graphviz", "d3"])
                .required(true),
        )
        .get_matches();

    let environment = matches
        .value_of("environment")
        .unwrap_or_else(|| panic!("environment is required"));
    let base_file_path = matches
        .value_of("base_file_path")
        .unwrap_or_else(|| panic!("base path file path is required"));
    let topic_file_name = matches
        .value_of("topic_file_name")
        .unwrap_or_else(|| panic!("topic file name is required"));
    let subscription_file_name = matches
        .value_of("subscription_file_name")
        .unwrap_or_else(|| panic!("subscription file name is required"));
    let output_format = matches.value_of("output_format").unwrap_or("json");

    Arg {
        environment: environment.into(),
        base_file_path: base_file_path.into(),
        topic_file_name: topic_file_name.into(),
        subscription_file_name: subscription_file_name.into(),
        output_format: output_format.into(),
    }
}
