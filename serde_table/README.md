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

## Why?
When you're writing tests, have you ever felt the desire to hide the data off in a file? Or switch to [ron](https://github.com/ron-rs/ron)?

This is because modern programming is preoccupied with the individual, not the batch. Here's a before, 153 characters.
```rust
    vec![
        Person {
            name: "John".to_string(),
            age: 50,
            city: "NewYork".to_string(),
        },
        Person {
            name: "Jane".to_string(),
            age: derive_age(),
            city: "LosAngeles".to_string(),
        },
    ]
```

After, 88 characters. That is a 2x improvement in signal-to-noise.
```rust
    let people: Vec<Person> = serde_table! {
        name    age             city
        John    30              NewYork
        Jane    derive_age()    LosAngeles
    }.unwrap();
```

## How does it work?
You should read the (small) [source](../serde_table_internals/src/lib.rs) for details, but the gist is that we
translate what you wrote into a CSV string, then parse that with [csv](https://docs.rs/csv/latest/csv/) / serde.
This can definitely be simplified, PRs welcome.