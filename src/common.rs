use std::env;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub fn read_input_as_rows_strings(requested: &str) -> Vec<String> {
    let f = find_root(env::current_dir().unwrap(), requested);
    fs::read_to_string(f)
        .expect("Input file not found")
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}

pub fn read_input_as_rows<T>(requested: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let f = find_root(env::current_dir().unwrap(), requested);
    fs::read_to_string(f)
        .expect("Input file not found")
        .lines()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn read_input_as_csv<T>(requested: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let f = find_root(env::current_dir().unwrap(), requested);
    fs::read_to_string(f)
        .expect("Input file not found")
        .split(',')
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn find_root(mut path: PathBuf, requested: &str) -> PathBuf {
    loop {
        path.push(".git");
        if path.exists() && path.is_dir() {
            path.pop();
            path.push("src");
            path.push(requested);
            break;
        } else {
            path.pop();
            path.pop();
        }
    }
    path
}

pub fn if_smaller(best_so_far: Option<i32>, candidate: i32) -> Option<i32> {
    match best_so_far {
        Some(m) => {
            if candidate < m {
                Some(candidate)
            } else {
                best_so_far
            }
        }
        None => Some(candidate),
    }
}
