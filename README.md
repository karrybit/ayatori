# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

###### May be BNF

```BNF
<resources>     ::= (<resource>)+
<resource>      ::= "resource" """ <resource_kind> """ """ <ident> """ "{" <attributes> "}"
<resource_kind> ::= aws_sns_topic | aws_sns_topic_subscription
<attributes>    ::= (<attribute>)*
<attribute>     ::= <ident> "=" (<here_doc> | <value>)
<ident>         ::= <string>*(<string>|<symbol>|<number>)*
<hear_doc>      ::= "<<" <tag> <dictionary> <tag>
<tag>           ::= ("A"|...|"Z")*
<json>          ::= "{" <dictionary> "}"
<dictionary>    ::= (""" <ident> """ "=" <value> ",")+ | (""" <ident> """ "=" <value>)
<value>         ::= <dictionary> | <array> | <atom>
<array>         ::= "[" ((<value> ",")+ | <value>) "]"
<atom>          ::= (""" <string>(<symbol>|<string>)+ """) | <number> | <bool>
<string>        ::= ("A"|...|"z")
<symbol>        ::= ("."|":"|"-"|"_")
<number>        ::= (1|...|9)+(0|...|9)
<bool>          ::= true | false
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
