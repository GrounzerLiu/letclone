# letclone

[![Crates.io](https://img.shields.io/crates/v/letclone.svg)](https://crates.io/crates/letclone)
[![Documentation](https://docs.rs/letclone/badge.svg)](https://docs.rs/letclone)
[![License](https://img.shields.io/crates/l/letclone.svg)](https://github.com/GrounzerLiu/letclone/blob/main/LICENSE)

A procedural macro for convenient variable cloning in Rust.

## Overview

`letclone` provides a `clone!` macro that simplifies the common pattern of cloning variables into new bindings. Instead of writing verbose `let` statements with `.clone()` calls, you can use the concise `clone!` macro.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
letclone = "0.1.0"
```

## Usage

### Basic Usage

```rust
use letclone::clone;

let original = String::from("hello");
clone!(original);
// Equivalent to: let original = original.clone();
```

### Field Access

```rust
use letclone::clone;

struct Person {
    name: String,
}

let person = Person {
    name: String::from("Alice"),
};
clone!(person.name);
// Equivalent to: let name = person.name.clone();

assert_eq!(name, "Alice");
```

### Method Call

```rust
use letclone::clone;

struct Container {
    value: String,
}

impl Container {
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

let container = Container {
    value: String::from("test"),
};
clone!(container.get_value());
// Equivalent to: let get_value = container.get_value().clone();

assert_eq!(get_value, "test");
```

### Mutable Bindings

Use the `mut` modifier to create mutable bindings:

```rust
use letclone::clone;

let original = String::from("hello");
clone!(mut original);
// Equivalent to: let mut original = original.clone();

original.push_str(" world");
assert_eq!(original, "hello world");
```

### Multiple Expressions

Clone multiple variables in a single macro call:

```rust
use letclone::clone;

struct Data {
    field1: String,
    field2: String,
}

let data = Data {
    field1: String::from("a"),
    field2: String::from("b"),
};
let var = String::from("c");

clone!(data.field1, data.field2, var);
// Equivalent to:
// let field1 = data.field1.clone();
// let field2 = data.field2.clone();
// let var = var.clone();

assert_eq!(field1, "a");
assert_eq!(field2, "b");
assert_eq!(var, "c");
```

## Supported Expression Types

| Expression Type | Example | Expands To |
|-----------------|---------|------------|
| Path/Variable | `clone!(var)` | `let var = var.clone();` |
| Field Access | `clone!(obj.field)` | `let field = obj.field.clone();` |
| Method Call | `clone!(obj.method())` | `let method = obj.method().clone();` |
| With `mut` | `clone!(mut var)` | `let mut var = var.clone();` |

## Limitations

- Tuple index access (e.g., `tuple.0`) is not supported
- Only named fields and method calls are supported for expression derivation

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
