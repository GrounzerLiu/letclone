// Test: clone! with complex nested expressions
use letclone::clone;

struct A {
    b: B,
}

struct B {
    tuple: (C, C),
}

#[derive(Clone)]
struct C {
    value: String,
}

impl C {
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

fn main() {
    let a = A {
        b: B {
            tuple: (
                C {
                    value: String::from("first"),
                },
                C {
                    value: String::from("second"),
                },
            ),
        },
    };

    // Test nested tuple index with field access
    clone!(a.b.tuple.0);
    clone!(a.b.tuple.1);

    assert_eq!(field_0.value, "first");
    assert_eq!(field_1.value, "second");

    // Test nested method call (only test one to avoid variable name conflict)
    clone!(a.b.tuple.0.get_value());
    assert_eq!(get_value, "first");
}
