// Test: clone! with multiple expressions
use letclone::clone;

struct Data {
    field1: String,
    field2: String,
}

fn main() {
    let data = Data {
        field1: String::from("a"),
        field2: String::from("b"),
    };
    let var = String::from("c");

    clone!(data.field1, data.field2, var);

    assert_eq!(field1, "a");
    assert_eq!(field2, "b");
    assert_eq!(var, "c");
}
