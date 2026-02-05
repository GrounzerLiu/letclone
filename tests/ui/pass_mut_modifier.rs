// Test: clone! with mut modifier
use letclone::clone;

fn main() {
    let original = String::from("hello");
    clone!(mut original);
    original.push_str(" world");
    assert_eq!(original, "hello world");
}
