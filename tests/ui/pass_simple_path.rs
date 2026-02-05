// Test: clone! with simple variable path
use letclone::clone;

fn main() {
    let original = String::from("hello");
    clone!(original);
    assert_eq!(original, "hello");
}
