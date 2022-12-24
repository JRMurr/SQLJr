# sqlJr
Toy DB to get more systems experience


## TODO

- General
  - Make a schema aware row struct
- Repl
  - Handle multiline queries (maybe streaming nom might work?)
- Parser
  - Would be cool to make a combinator/macro that is tuple where each elem is separated by whitespace
  - Display errors nicely (look into what swc/rustc does to make it look good)
  - Would be cool to make a macro for SqlQuery that auto makes the alt with each variant

## Resources
- [Nom sql parser](https://github.com/ms705/nom-sql)
- [DB walk through](https://cstack.github.io/db_tutorial/)
- [Nom combinators](https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md)
- [Searlization/page stuff](https://www.reddit.com/r/rust/comments/ukz786/rust_way_of_dealing_with_memory_allocation/)