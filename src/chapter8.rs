use std::collections::HashMap;

pub mod hashmap_demo;
pub mod string_demo;
pub mod vector_demo;

pub fn main() {
    // vector_demo::main();
    // string_demo::main();
    hashmap_demo::main();
}

#[test]
fn hashmap_demo() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}
