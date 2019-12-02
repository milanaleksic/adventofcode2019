mod common;

#[path = "advent01/lib.rs"]
mod advent01;

#[path = "advent02/lib.rs"]
mod advent02;

pub fn all_solvers() -> Vec<Box<dyn common::Solver>> {
    vec![
        Box::new(advent01::Solver{}),
        Box::new(advent02::Solver{}),
    ]
}