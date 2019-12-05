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

pub fn all_solvers() -> Vec<Box<dyn common::Solver>> {
    vec![
        Box::new(advent01::Solver {}),
        Box::new(advent02::Solver {}),
        Box::new(advent03::Solver {}),
        Box::new(advent04::Solver {}),
        Box::new(advent05::Solver {}),
    ]
}
