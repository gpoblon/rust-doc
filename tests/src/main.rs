/*
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Millimeters(u32);
#[derive(Debug,PartialEq)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let my_string = String::from("hello world");
    let word = &my_string[..];

    println!("string: {}", word);
    assert_eq!(Millimeters(1) + Millimeters(4), Millimeters(5));
}
*/

use std::thread;

fn main() {
    let mut v = 1;

    let handle = thread::spawn(|| {
        let x = v;
    });
    handle.join();
}