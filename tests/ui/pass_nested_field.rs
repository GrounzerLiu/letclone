// Test: clone! with nested field access
use letclone::clone;

struct A {
    b: B,
}

struct B {
    c: C,
}

struct C {
    d: String,
}

fn main() {
    let a = A {
        b: B {
            c: C {
                d: String::from("nested"),
            },
        },
    };

    clone!(a.b.c.d);
    assert_eq!(d, "nested");
}
