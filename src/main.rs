use adventofcode2019::Solver;
use std::env;
use std::time::Instant;

fn main() {
    let all_solvers = adventofcode2019::all_solvers();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == String::from("last") {
        all_solvers.last().map(|s| run_solver(s));
    } else {
        all_solvers.iter().for_each(|s| run_solver(s));
    }
}

fn run_solver(s: &Box<dyn Solver>) -> () {
    println!("===\nsolver: {}", s.name());
    let start = Instant::now();
    let (answer1, answer2) = s.solve_all();
    let elapsed = start.elapsed();
    println!(
        "answer 1: {}\nanswer 2: {}\ntime: {}ms",
        answer1,
        answer2,
        elapsed.as_millis()
    );
}
