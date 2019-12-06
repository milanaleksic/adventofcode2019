mod common;

#[path = "advent01/lib.rs"]
mod advent01;

#[path = "advent02/lib.rs"]
mod advent02;

#[path = "advent03/lib.rs"]
mod advent03;

#[path = "advent04/lib.rs"]
mod advent04;

#[path = "advent05/lib.rs"]
mod advent05;

#[path = "advent06/lib.rs"]
mod advent06;

pub trait Solver {
    fn name(&self) -> &str;
    fn solve_all(&self) -> (String, String) {
        (self.solve_a(), self.solve_b())
    }
    fn solve_a(&self) -> String;
    fn solve_b(&self) -> String;
}

pub fn all_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(advent01::Solver {}),
        Box::new(advent02::Solver {}),
        Box::new(advent03::Solver {}),
        Box::new(advent04::Solver {}),
        Box::new(advent05::Solver {}),
        Box::new(advent06::Solver {}),
    ]
}
