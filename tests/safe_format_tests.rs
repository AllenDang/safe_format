extern crate safe_format;
use safe_format::safe_format;

#[test]
fn test_safe_format_literal() {
    let result = safe_format!("Hello, {name}!", name = "Alice");
    assert_eq!(result, "Hello, Alice!");
}

#[test]
fn test_safe_format_variable() {
    let pattern = "Greeting from {name}";
    let result = safe_format!(pattern, name = "Allen");
    assert_eq!(result, "Greeting from Allen");
}

#[test]
fn test_safe_format_unused_args() {
    let pattern = "Greeting from {name}";
    let result = safe_format!(pattern, name = "Allen", unused = "Unused");
    assert_eq!(result, "Greeting from Allen");
}

#[test]
fn test_safe_format_multiple_args() {
    let pattern = "{greeting}, {name}!";
    let result = safe_format!(pattern, greeting = "Hello", name = "Bob");
    assert_eq!(result, "Hello, Bob!");
}

#[test]
fn test_safe_format_dynamic_string() {
    let dynamic_pattern = String::from("Welcome, {user}!");
    let result = safe_format!(dynamic_pattern, user = "Charlie");
    assert_eq!(result, "Welcome, Charlie!");
}
