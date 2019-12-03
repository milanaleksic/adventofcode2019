use std::env;

fn main() {
    let all_solvers = adventofcode2019::all_solvers();
    let args: Vec<String> = env::args().collect();
    if args.len()==2 && args[1] == String::from("last") {
        all_solvers.last().map(|s| {
            println!("===\nsolver: {}", s.name());
            let (answer1, answer2) = s.solve_all();
            println!("answer 1: {}\nanswer 2: {}", answer1, answer2);
        });
    } else {
        all_solvers.iter().for_each(|s| {
            println!("===\nsolver: {}", s.name());
            let (answer1, answer2) = s.solve_all();
            println!("answer 1: {}\nanswer 2: {}", answer1, answer2);
        });
    }
}
