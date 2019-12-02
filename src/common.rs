use std::env;
use std::fs;

pub trait Solver {
    fn name(&self) -> &str;
    fn solve_a(&self) -> String;
    fn solve_b(&self) -> String;
}

pub fn read_input_as_ints(path: &str) -> Vec<i32> {
    let current_dir = env::current_dir().unwrap();
    let last_dir = current_dir.file_name().unwrap();
    let f = if last_dir == "src" { String::from(path) } else { String::from("src/") + path };
    fs::read_to_string(f)
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn read_input_as_csv_ints(path: &str) -> Vec<usize> {
    let current_dir = env::current_dir().unwrap();
    let last_dir = current_dir.file_name().unwrap();
    let f = if last_dir == "src" { String::from(path) } else { String::from("src/") + path };
    fs::read_to_string(f)
        .expect("Input file not found")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}