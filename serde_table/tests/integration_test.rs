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
fn test_motivation() {
    let people: Vec<Person> = serde_table! {
        name    age   city
        John    30    NewYork
        Jane    25    LosAngeles
    }
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
fn test_motivation_comments() {
    let people: Vec<Person> = serde_table! {
        name    age   city
        // John    30    NewYork
        Jane    25    LosAngeles
    }
    .unwrap();

    assert_eq!(
        people,
        vec![
            // Person {
            //     name: "John".to_string(),
            //     age: 30,
            //     city: "NewYork".to_string(),
            // },
            Person {
                name: "Jane".to_string(),
                age: 25,
                city: "LosAngeles".to_string(),
            },
        ]
    );
}

#[test]
fn test_quoted_whitespace_base() {
    let people: Vec<Person> = serde_table! {
        name     age      city
        "Alice with a space"    42       Seattle
        "Bob"      "38"       "Portland"
    }
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
fn test_quoted_whitespace_expr() {
    let people: Vec<Person> = serde_table_expr! {
        "name"     "age"      "city"
        "Alice with a space"    42       "Seattle"
        "Bob"      "38"       "Portland"
    }
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
fn test_exprs_base() {
    let calc_age = |_| 24;
    let people: Vec<Person> = serde_table! {
        "name"     "age"         city
        "Alice"    42            "Seattle"
        "Bob"      calc_age("hi")    "Portland"
    }
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

#[test]
fn test_exprs_expr() {
    let calc_age = |_| 24;
    let people: Vec<Person> = serde_table_expr! {
        "name"     "age"         "city"
        "Alice"    42            "Seattle"
        "Bob"      calc_age("hi")    "Portland"
    }
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

#[test]
fn test_base_magic() {
    let p = Person {
        name: "John".to_string(),
        age: 30,
        city: "NewYork".to_string(),
    };
    // John    p.name.to_uppercase()    NewYork
    let people: Vec<Person> = serde_table! {
        name    age   city
        John    p.name    NewYork
        Jane       LosAngeles
    }
    .unwrap();
}
