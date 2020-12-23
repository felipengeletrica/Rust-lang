use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    map.insert(1, 2);
    map.insert(2, 2);

    println!("Map:{:?}", map);
    println!("Secrete number: {}", secret_number);
}