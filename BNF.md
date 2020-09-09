###### May be BNF

```
<resources>             ::= (<resource>)+
<resource>              ::= 'resource' <string> <string> '{' <attribute>* '}'
<resource_kind>         ::= 'aws_sns_topic' | 'aws_sns_topic_subscription'
<attribute>             ::= <ident> '=' (<value> | <headoc>)
<value>                 ::= <dictionary> | <atom> | <ident>
<dictionary>            ::= '{' (<ident> '=' <value>)+ '}'
<ident>                 ::= <char>(<char> | <symbol> | <number>)*

<heardoc>               ::= '<<' <tag> <dictionary> <tag>
<heardoc_tag>           ::= ('A'|...|'Z')*
<heardoc_dictionary>    ::= '{' <key_values> '}'
<heardoc_key_values>    ::= <key_value> | <key_value> ',' <key_values>
<heardoc_key_value>     ::= <string> ':' '=' <value>
<heardoc_array>         ::= '[' <values> ']'
<heardoc_values>        ::= <value> | <value> ',' <values>
<heardoc_value>         ::= <dictionary> | <array> | <atom>

<atom>                  ::= <string> | <number> | <bool>
<string>                ::= '"' (<char>(<char> | <symbol> | <number>)*)+ '"'
<char>                  ::= ('A'|...|'z')
<symbol>                ::= ('.'|':'|'-'|'_')
<number>                ::= (1|...|9)+(0|...|9)
<bool>                  ::= true | false
```
