# letclone

[![Crates.io](https://img.shields.io/crates/v/letclone.svg)](https://crates.io/crates/letclone)
[![Documentation](https://docs.rs/letclone/badge.svg)](https://docs.rs/letclone)
[![License](https://img.shields.io/crates/l/letclone.svg)](https://github.com/GrounzerLiu/letclone/blob/main/LICENSE)

A procedural macro for convenient variable cloning in Rust.

## Overview

`letclone` provides a `clone!` macro that simplifies the common pattern of cloning variables into new bindings. Instead of writing verbose `let` statements with `.clone()` calls, you can use the concise `clone!` macro.

The macro is especially useful when working with closures that need to capture cloned values, as it reduces boilerplate code significantly.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
letclone = "0.2.0"
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

### Tuple Index Access

```rust
use letclone::clone;

let tuple = (String::from("first"), String::from("second"));
clone!(tuple.0, tuple.1);
// Equivalent to:
// let field_0 = tuple.0.clone();
// let field_1 = tuple.1.clone();

assert_eq!(field_0, "first");
assert_eq!(field_1, "second");
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

Use the `mut` modifier to create mutable bindings for variables, fields, and tuple indices:

```rust
use letclone::clone;

// Variable
let original = String::from("hello");
clone!(mut original);
// Equivalent to: let mut original = original.clone();
original.push_str(" world");
assert_eq!(original, "hello world");

// Field access
struct Data {
    value: String,
}
let data = Data { value: String::from("test") };
clone!(mut data.value);
// Equivalent to: let mut value = data.value.clone();
value.push_str(" updated");
assert_eq!(value, "test updated");

// Tuple index
let tuple = (String::from("first"), String::from("second"));
clone!(mut tuple.0);
// Equivalent to: let mut field_0 = tuple.0.clone();
field_0.push_str(" item");
assert_eq!(field_0, "first item");
```

### Nested Field Access

```rust
use letclone::clone;

struct A {
    b: B,
}

struct B {
    c: String,
}

let a = A {
    b: B {
        c: String::from("nested"),
    },
};

clone!(a.b.c);
// Equivalent to: let c = a.b.c.clone();

assert_eq!(c, "nested");
```

### Nested Tuple Index Access

```rust
use letclone::clone;

struct Container {
    tuple: (String, String),
}

let container = Container {
    tuple: (String::from("first"), String::from("second")),
};

clone!(container.tuple.0);
clone!(container.tuple.1);
// Equivalent to:
// let field_0 = container.tuple.0.clone();
// let field_1 = container.tuple.1.clone();

assert_eq!(field_0, "first");
assert_eq!(field_1, "second");
```

### Nested Method Call

```rust
use letclone::clone;

struct Outer {
    inner: Inner,
}

struct Inner {
    value: String,
}

impl Inner {
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

let outer = Outer {
    inner: Inner {
        value: String::from("nested method"),
    },
};

clone!(outer.inner.get_value());
// Equivalent to: let get_value = outer.inner.get_value().clone();

assert_eq!(get_value, "nested method");
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

### Usage in Closures

The `clone!` macro is particularly useful when working with closures that need to capture cloned values:

```rust
use letclone::clone;

let name = String::from("Alice");
let scores = vec![85, 90, 95];

// Without clone! macro
let closure1 = {
    let name = name.clone();
    let scores = scores.clone();
    move || {
        println!("Name: {}, Scores: {:?}", name, scores);
    }
};

// With clone! macro - much cleaner!
let closure2 = {
    clone!(name, scores);
    move || {
        println!("Name: {}, Scores: {:?}", name, scores);
    }
};

closure1();
closure2();
```

For nested structures in closures:

```rust
use letclone::clone;

struct Person {
    name: String,
    address: Address,
}

struct Address {
    street: String,
    city: String,
}

let person = Person {
    name: String::from("Bob"),
    address: Address {
        street: String::from("123 Main St"),
        city: String::from("New York"),
    },
};

let closure = {
    clone!(person.name, person.address.street, person.address.city);
    move || {
        println!("{} lives at {}, {}", name, street, city);
    }
};

closure();
```

## Supported Expression Types

| Expression Type | Example | Expands To |
|-----------------|---------|------------|
| Path/Variable | `clone!(var)` | `let var = var.clone();` |
| Field Access | `clone!(obj.field)` | `let field = obj.field.clone();` |
| Nested Field Access | `clone!(a.b.c)` | `let c = a.b.c.clone();` |
| Tuple Index | `clone!(tuple.0)` | `let field_0 = tuple.0.clone();` |
| Nested Tuple Index | `clone!(obj.tuple.0)` | `let field_0 = obj.tuple.0.clone();` |
| Method Call | `clone!(obj.method())` | `let method = obj.method().clone();` |
| Nested Method Call | `clone!(a.b.method())` | `let method = a.b.method().clone();` |
| With `mut` | `clone!(mut var)` | `let mut var = var.clone();` |
| With `mut` on field | `clone!(mut obj.field)` | `let mut field = obj.field.clone();` |
| With `mut` on tuple | `clone!(mut tuple.0)` | `let mut field_0 = tuple.0.clone();` |

## Limitations

The `clone!` macro supports a specific set of expression types for automatic variable name derivation:

**Supported:**
- Path expressions (variables): `clone!(var)`
- Field access: `clone!(obj.field)`, `clone!(a.b.c)` (nested)
- Tuple index access: `clone!(tuple.0)`, `clone!(obj.tuple.0)` (nested)
- Method calls: `clone!(obj.method())`, `clone!(a.b.method())` (nested)

**Not supported:**
- Binary expressions: `clone!(a + b)`
- Literals: `clone!(42)`
- Array indexing: `clone!(arr[0])`
- Complex expressions that cannot derive a variable name automatically

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
