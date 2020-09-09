# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

<image src="./img/dependency.svg">

### Requirement

##### tool

- graphviz

##### file tree

`<target_base_path>/<service_name>/<environment>/<topic_file_name>`
`<target_base_path>/<service_name>/<environment>/<subscription_file_name>`

```sh
$ tree analyze_target
analyze_target
├── service_A
│    ├── develop
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    ├── staging
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    └── production
│         ├── sns_topic.tf
│         └── sns_subscription.tf
├── service_B
│    ├── develop
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    ├── staging
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    └── production
│         ├── sns_topic.tf
│         └── sns_subscription.tf
├── service_C
│    ├── develop
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    ├── staging
│    │    ├── sns_topic.tf
│    │    └── sns_subscription.tf
│    └── production
│         ├── sns_topic.tf
│         └── sns_subscription.tf
```

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
