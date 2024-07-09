# safe_format

`safe_format` is a procedural macro for Rust that allows you to safely format strings with named parameters. It works similarly to the built-in `format!` macro but allows for named parameters, and it safely ignores any extra parameters that are not used in the format string.

## Features

- Named parameters for string formatting.
- Ignores extra parameters that are not used in the format string.
- Simple and intuitive API.

## Usage

```rust
use safe_format::safe_format;

fn main() {
    let name = "Allen";
    let age = 19;
    let greeting = safe_format!("Greeting from {name}", name = name, age = age);
    println!("{}", greeting);  // Output: Greeting from Allen
}

```

### Add Dependency

First, add the `safe_format_macro` crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
safe_format = 0.1
```
