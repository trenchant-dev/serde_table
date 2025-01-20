# serde_table
A macro for parsing tables into Rust structs.

```rust
use serde::Deserialize;
use serde_table::serde_table;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

let people: Vec<Person> = serde_table! {
    name    age   city
    John    30    NewYork
    Jane    25    LosAngeles
}.unwrap();
```

## Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
serde_table = "0.1.0"
```


## Advanced Usage
While `serde_table` ought to do the right thing in general,
you can use `serde_table_expr` if you need to avoid the automatic quoting of bare variable-names (identifiers).
