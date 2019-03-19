use std::collections::HashMap;

struct People(HashMap<i32, String>);
#[derive(Debug)]
struct Years(i64);

fn main() {

    let age = Years(5);
    println!("Hello, world! {:?}", age);
}
