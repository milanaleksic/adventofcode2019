use advent01::{fuel2, Fuel};
use std::fs;

fn main() {
    let answer1: i32 = read_input_as_ints("src/input.txt")
        .iter()
        .map(|x| Fuel::fuel(x.clone()))
        .sum();
    println!("answer 1: {}", answer1);
    let answer2: i32 = read_input_as_ints("src/input2.txt")
        .iter()
        .map(|x| fuel2(x.clone()))
        .sum();
    println!("answer2: {}", answer2);
}

fn read_input_as_ints(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
