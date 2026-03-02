// Test: clone! with nested method call
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

fn main() {
    let outer = Outer {
        inner: Inner {
            value: String::from("nested method"),
        },
    };

    clone!(outer.inner.get_value());
    assert_eq!(get_value, "nested method");
}
