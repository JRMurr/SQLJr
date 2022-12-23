# sqlJr
Toy DB to get more systems experience


## TODO

- Repl
  - Handle multiline queries (maybe streaming nom might work?)
- Parser
  - Would be cool to make a combinator/macro that is tuple where each elem is separated by whitespace
  - Make a trait that each command can implement to be parsed from a span (like gdlk...)

## Resources
- [Nom sql parser](https://github.com/ms705/nom-sql)
- [DB walk through](https://cstack.github.io/db_tutorial/)
- [Nom combinators](https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md)
- [Searlization/page stuff](https://www.reddit.com/r/rust/comments/ukz786/rust_way_of_dealing_with_memory_allocation/)