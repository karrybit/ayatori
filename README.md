# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

###### May be BNF

```
<resources>     ::= (<resource>)+
<resource>      ::= 'resource' <string> <string> '{' <attribute>* '}'
<resource_kind> ::= 'aws_sns_topic' | 'aws_sns_topic_subscription'
<attribute>     ::= <ident> '=' (<atom> | <ident> | <headoc>)
<ident>         ::= <char>(<char> | <symbol> | <number>)*
<hear_doc>      ::= '<<' <tag> <dictionary> <tag>
<tag>           ::= ('A'|...|'Z')*

<dictionary>    ::= '{' <key_values> '}'
<key_values>    ::= <key_value> | <key_value> ',' <key_values>
<key_value>     ::= <string> ':' '=' <value>
<array>         ::= '[' <values> ']'
<values>        ::= <value> | <value> ',' <values>
<value>         ::= <dictionary> | <array> | <atom>
<atom>          ::= <string> | <number> | <bool>

<string>        ::= '"' (<char>(<char> | <symbol> | <number>)*)+ '"'
<char>          ::= ('A'|...|'z')
<symbol>        ::= ('.'|':'|'-'|'_')
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
