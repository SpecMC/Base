# SpecMC Base
A library with common code for parsing Minecraft specification.

## Examples
```rust
use specmc_base::{
    parse::{Identifier, Literal},
    tokenize::tokenize,
};

let mut tokens = tokenize("true 42 123.0 \"string\" cool_identifier");
tokens.reverse();
let lit_bool = Literal::parse(&mut tokens).unwrap();
let lit_int = Literal::parse(&mut tokens).unwrap();
let lit_float = Literal::parse(&mut tokens).unwrap();
let lit_str = Literal::parse(&mut tokens).unwrap();
let ident = Identifier::parse(&mut tokens).unwrap();
println!("{lit_bool:?} {lit_int:?} {lit_float:?} {lit_str:?} {ident:?}");
```
