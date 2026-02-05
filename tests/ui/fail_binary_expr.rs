// Test: clone! with binary expression should fail
use letclone::clone;

fn main() {
    let a = 1;
    let b = 2;
    clone!(a + b);
}
