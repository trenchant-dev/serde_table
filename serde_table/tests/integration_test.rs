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
fn test_base_invalid() {
    let p = Person {
        name: "John".to_string(),
        age: 30,
        city: "NewYork".to_string(),
    };
    // John    p.name.to_uppercase()    NewYork
    let result: Result<Vec<Person>, _> = serde_table! {
        name    age   city
        John    p.name    NewYork
        Jane    24   LosAngeles
    };
    // InvalidDigit error
    assert!(result.is_err());
}

/// I don't know why you'd want to do anything in here, but at least the errors I found while tinkering in this space are good.
#[test]
fn test_dumb_magic() {
    let p = Person {
        name: "John".to_string(),
        age: 100,
        city: "NewYork".to_string(),
    };
    // John    p.name.to_uppercase()    NewYork
    let people: Vec<Person> = serde_table! {
        name    age   city
        John    p.age.saturating_sub(if p.age >= 100 { 50 } else { 0 })    NewYork
        Jane    24   LosAngeles
    }
    .unwrap();
    assert_eq!(
        people,
        vec![
            Person {
                name: "John".to_string(),
                age: 50,
                city: "NewYork".to_string(),
            },
            Person {
                name: "Jane".to_string(),
                age: 24,
                city: "LosAngeles".to_string(),
            },
        ]
    );

    let p = Person {
        name: "John".to_string(),
        age: 100,
        city: "NewYork".to_string(),
    };
    let people: Vec<Person> = serde_table_expr! {
        "name"    "age"   "city"
        "John"    p.age.saturating_sub(if p.age >= 100 { 50 } else { 0 })    "NewYork"
        // This gives a pretty good error, Rust is amazing.
        // "Jane"    (24-1)   "LosAngeles"
        "Jane"   24-1   "LosAngeles"

        // InvalidDigit error, nice.
        // "John" p.age "NewYork"
    }
    .unwrap();
    assert_eq!(
        people,
        vec![
            Person {
                name: "John".to_string(),
                age: 50,
                city: "NewYork".to_string(),
            },
            Person {
                name: "Jane".to_string(),
                age: 23,
                city: "LosAngeles".to_string(),
            },
        ]
    );
}

/// I don't know why you'd want to do anything in here, but at least the errors I found while tinkering in this space are good.
/// Copy paste of test_dumb_magic but with the base macro.
#[test]
fn test_dumb_magic_base() {
    let p = Person {
        name: "John".to_string(),
        age: 100,
        city: "NewYork".to_string(),
    };
    // John    p.name.to_uppercase()    NewYork
    let people: Vec<Person> = serde_table! {
        name    age   city
        John    p.age.saturating_sub(if p.age >= 100 { 50 } else { 0 })    NewYork
        Jane    24   LosAngeles
    }
    .unwrap();
    assert_eq!(
        people,
        vec![
            Person {
                name: "John".to_string(),
                age: 50,
                city: "NewYork".to_string(),
            },
            Person {
                name: "Jane".to_string(),
                age: 24,
                city: "LosAngeles".to_string(),
            },
        ]
    );

    let p = Person {
        name: "John".to_string(),
        age: 100,
        city: "NewYork".to_string(),
    };
    let people: Vec<Person> = serde_table! {
        "name"    "age"   "city"
        "John"    p.age.saturating_sub(if p.age >= 100 { 50 } else { 0 })    "NewYork"
        // This gives a pretty good error, Rust is amazing.
        // "Jane"    (24-1)   "LosAngeles"
        // You can argue that 24-1 should have been preserved as a string, but it's syntax highlighted as Rust code, so you'd be wrong.
        "Jane"   24-1   "LosAngeles"

        // InvalidDigit error, nice.
        // "John" p.age "NewYork"
    }
    .unwrap();
    assert_eq!(
        people,
        vec![
            Person {
                name: "John".to_string(),
                age: 50,
                city: "NewYork".to_string(),
            },
            Person {
                name: "Jane".to_string(),
                age: 23,
                city: "LosAngeles".to_string(),
            },
        ]
    );
}
