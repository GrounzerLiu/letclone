// Test: clone! with field access
use letclone::clone;

struct Person {
    name: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    clone!(person.name);
    assert_eq!(name, "Alice");
}
