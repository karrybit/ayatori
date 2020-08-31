# ayatori

### Analysis of dependency between services in microservices

This app parse Terraform writing relationship of microservices. Then, solve SNS topic and subscription as dependency graph

###### May be BNF

```
<statements>    ::= (<statement>)+
<statement>     ::= "resource" <resource_name> <event_name> <attributes>
<attributes>    ::= "{" ((<attribute> ",")+ | <attribute>) "}"
<attribute>     ::= <key> "=" (<here_doc> | <value>)
<here_doc>      ::= "<<" <atom> <attributs> <atom>
<value>         ::= <atom> | "[" <atoms> "]" | <attributes>
<array>         ::= "[" <atoms> "]"
<atoms>         ::= (<atom> ",")+ | <atom>
<resource_name> ::= <atom>
<event_name>    ::= <atom>
<key>           ::= <atom>
<atom>          ::= ("a"|...|"z")*("a"|...|"z"|"."|"-"|"_"|"0"|...|"9")*
```
