// Test: clone! usage in closures
use letclone::clone;

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let scores = vec![85, 90, 95];

    // Test basic closure usage
    let closure1 = {
        clone!(person.name, scores);
        move || format!("{}: {:?}", name, scores)
    };

    let result1 = closure1();
    assert_eq!(result1, "Alice: [85, 90, 95]");

    // Test with mutable clone
    let counter = 0;
    let mut closure2 = {
        clone!(mut counter);
        move || {
            counter += 1;
            counter
        }
    };

    assert_eq!(closure2(), 1);
    assert_eq!(closure2(), 2);

    // Test nested field in closure
    struct Outer {
        inner: Inner,
    }

    struct Inner {
        value: String,
    }

    let outer = Outer {
        inner: Inner {
            value: String::from("nested"),
        },
    };

    let closure3 = {
        clone!(outer.inner.value);
        move || value.clone()
    };

    assert_eq!(closure3(), "nested");
}
