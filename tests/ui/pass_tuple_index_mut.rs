// Test: clone! with tuple index access and mut modifier
use letclone::clone;

fn main() {
    let tuple = (String::from("hello"), String::from("world"));
    clone!(mut tuple.0);
    clone!(mut tuple.1);

    field_0.push_str(" there");
    field_1.push_str("!");

    assert_eq!(field_0, "hello there");
    assert_eq!(field_1, "world!");
}
