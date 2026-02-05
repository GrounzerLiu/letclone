// Test: clone! with method call
use letclone::clone;

struct Container {
    value: String,
}

impl Container {
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

fn main() {
    let container = Container {
        value: String::from("test"),
    };
    clone!(container.get_value());
    assert_eq!(get_value, "test");
}
