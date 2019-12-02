use std::env;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub trait Solver {
    fn name(&self) -> &str;
    fn solve_a(&self) -> String;
    fn solve_b(&self) -> String;
}

pub fn read_input_as_rows<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let current_dir = env::current_dir().unwrap();
    let last_dir = current_dir.file_name().unwrap();
    let f = if last_dir == "src" {
        String::from(path)
    } else {
        String::from("src/") + path
    };
    fs::read_to_string(f)
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn read_input_as_csv<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let current_dir = env::current_dir().unwrap();
    let last_dir = current_dir.file_name().unwrap();
    let f = if last_dir == "src" {
        String::from(path)
    } else {
        String::from("src/") + path
    };
    fs::read_to_string(f)
        .expect("Input file not found")
        .split(',')
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
