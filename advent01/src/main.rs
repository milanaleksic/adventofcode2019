use advent01::{Fuel, fuel2};
use std::fs;

fn main() {
    println!("answer 1: {}", read_input_as_ints("src/input.txt")
        .iter()
        .map(|x| Fuel::fuel(x.clone()))
        .fold(0, |acc, x| acc + x));
    println!("answer2: {}", read_input_as_ints("src/input2.txt")
        .iter()
        .map(|x| fuel2(x.clone()))
        .fold(0, |acc, x| acc + x));
}

fn read_input_as_ints(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
