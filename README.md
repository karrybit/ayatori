# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

###### May be BNF

```
<resources>    ::= (<resource>)+
<resource>     ::= "resource" <resource_name> <atom> "{" <attributes> "}"
<resource_name> ::= "aws_sns_topic" | "aws_sns_topic_subscription"
<attributes>    ::= (<attribute>)*
<attribute>     ::= <key> "=" (<here_doc> | <value>)
<hear_doc>     ::= "<<" <tag> <dictionary> <tag>
<tag>           ::= ("A"|...|"Z")*
<dictionary>    ::= "{" (<key> "=" <value> ",")+ | (<key> "=" <value>) "}"
<key>           ::= <atom>
<value>         ::= <dictionary> | <array> | <atom>
<array>         ::= "[" ((<atoms> ",")+ | <atom>) "]"
<atom>          ::= ("a"|...|"z")*("a"|...|"z"|"."|"-"|"_"|"0"|...|"9")*
```

### Usage

```sh
USAGE:
    ayatori --environment <environment> --base_path <base_path> --topic <topic_file_name> --subscription <subscription_file_name>
OPTIONS:
    -e, --environment    <environment>                  Environment [possible values: develop, staging, production]
    -b, --base_file_path <base_file_path>               Base file path
    -t, --topic      <topic_file_name>                  Topic file name
    -s, --subscription     <subscription_file_name>     Subscription file name
```
