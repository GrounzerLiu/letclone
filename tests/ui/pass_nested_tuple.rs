// Test: clone! with nested tuple index access
use letclone::clone;

struct Container {
    tuple: (String, String),
}

fn main() {
    let container = Container {
        tuple: (String::from("first"), String::from("second")),
    };

    clone!(container.tuple.0);
    clone!(container.tuple.1);

    assert_eq!(field_0, "first");
    assert_eq!(field_1, "second");
}
