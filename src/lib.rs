mod common;

#[path = "advent01/lib.rs"]
mod advent01;

pub fn all_solvers() -> Vec<Box<dyn common::Solver>> {
    vec![
        Box::new(advent01::Solver{}),
    ]
}