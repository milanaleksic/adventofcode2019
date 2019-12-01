use advent01::{fuel, fuel2};
use std::fs;

fn main() {
    println!("answer 1: {}", fs::read_to_string("src/input.txt")
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|x| fuel(x))
        .fold(0, |acc, x| acc + x));
    println!("answer2: {}", fs::read_to_string("src/input2.txt")
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|x| fuel2(x))
        .fold(0, |acc, x| acc + x));
}
