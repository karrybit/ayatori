# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

### Requirement

- graphviz

### Usage

```sh
$ make help

USAGE:
    ayatori-driver [FLAGS] --base_file_path <base_file_path> --environment <environment> --subscription <subscription_file_name> --topic <topic_file_name>

FLAGS:
    -h, --help          Prints help information
    -c, --concurrent    run concurrent
    -V, --version       Prints version information

OPTIONS:
    -b, --base_file_path <base_file_path>          Base file path
    -e, --environment <environment>                Environment [possible values: develop, staging, production]
    -f, --format <output_format>                   Output format [possible values: json, dot, d3]
    -s, --subscription <subscription_file_name>    Subscription file name
    -t, --topic <topic_file_name>                  Topic file name
```
