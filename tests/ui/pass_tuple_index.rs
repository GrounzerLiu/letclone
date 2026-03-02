// Test: clone! with tuple index access
use letclone::clone;

fn main() {
    let tuple = (String::from("hello"), String::from("world"));
    clone!(tuple.0);
    clone!(tuple.1);
    assert_eq!(field_0, "hello");
    assert_eq!(field_1, "world");
}
