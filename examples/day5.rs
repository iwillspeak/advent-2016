extern crate advent;

use advent::day5::*;

const INPUT: &'static str = "ojvtpuvg";

pub fn main() {
    println!("{}: {}", INPUT, get_password(INPUT));
}
