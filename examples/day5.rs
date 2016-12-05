extern crate advent;

use advent::day5::*;

use std::thread;

const INPUT: &'static str = "ojvtpuvg";

pub fn main() {
    let basic = thread::spawn(|| get_password(INPUT));
    let indexed = thread::spawn(|| get_indexed_password(INPUT));
    println!("{} (basic): {}", INPUT, basic.join().unwrap());
    println!("{} (indexed): {}", INPUT, indexed.join().unwrap());
}
