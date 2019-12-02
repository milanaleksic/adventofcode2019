fn main() {
    adventofcode2019::all_solvers().iter().for_each(|s| {
        println!("===\nsolver: {}", s.name());
        println!("answer 1: {}", s.solve_a());
        println!("answer 2: {}", s.solve_b());
    });
}
