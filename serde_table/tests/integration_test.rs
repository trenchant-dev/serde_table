use serde_table::serde_table;
use serde_table::serde_table_expr;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

#[test]
fn test_basic_parsing() {
    let people: Vec<Person> = serde_table! {
        "name"    "age"   "city"
        "John"    30    "NewYork"
        "Jane"    25    "LosAngeles"
    }
    .parse()
    .unwrap();

    assert_eq!(
        people,
        vec![
            Person {
                name: "John".to_string(),
                age: 30,
                city: "NewYork".to_string(),
            },
            Person {
                name: "Jane".to_string(),
                age: 25,
                city: "LosAngeles".to_string(),
            },
        ]
    );
}

#[test]
fn test_flexible_whitespace() {
    let people: Vec<Person> = serde_table! {
        "name"     "age"      "city"
        "Alice with a space"    42       "Seattle"
        "Bob"      38       "Portland"
    }
    .parse()
    .unwrap();

    assert_eq!(
        people,
        vec![
            Person {
                name: "Alice with a space".to_string(),
                age: 42,
                city: "Seattle".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: 38,
                city: "Portland".to_string(),
            },
        ]
    );
}

#[test]
fn test_exprs() {
    let calc_age = |_| 24;
    let people: Vec<Person> = serde_table_expr! {
        "name"     "age"         city
        "Alice"    42            "Seattle"
        "Bob"      calc_age("hi")    "Portland"
    }
    .parse()
    .unwrap();

    assert_eq!(
        people,
        vec![
            Person {
                name: "Alice".to_string(),
                age: 42,
                city: "Seattle".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: calc_age("hi"),
                city: "Portland".to_string(),
            },
        ]
    );
}
