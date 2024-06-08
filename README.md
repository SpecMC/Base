# SpecMC Base

A library with common code for parsing Minecraft specification.

## Examples

```rust
use specmc_base::{
    parse::{Identifier, Literal, Parse},
    tokenize::tokenize,
};

let mut tokens: Vec<String> = tokenize("true 42 123.0 \"string\" cool_identifier");
tokens.reverse();

let lit_bool: Literal = Literal::parse(&mut tokens).unwrap();
let lit_int: Literal = Literal::parse(&mut tokens).unwrap();
let lit_float: Literal = Literal::parse(&mut tokens).unwrap();
let lit_str: Literal = Literal::parse(&mut tokens).unwrap();
let ident: Identifier = Identifier::parse(&mut tokens).unwrap();

println!("{lit_bool:?} {lit_int:?} {lit_float:?} {lit_str:?} {ident:?}");
```
