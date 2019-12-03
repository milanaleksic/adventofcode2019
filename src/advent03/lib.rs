use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::common::read_input_as_rows_strings;
use crate::common::if_smaller;

use super::common;
use std::fmt::{Display, Formatter, Error, Debug};

struct Cell {
    x: i32,
    y: i32,
    latency: i32,
}

impl Cell {
    fn new(x: i32, y: i32, latency: i32) -> Cell {
        Cell {
            x,
            y,
            latency,
        }
    }
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn latency(&self) -> i32 {
        self.latency
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cell {}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Solver {}

impl Solver {
    fn get_data(&self) -> Vec<Vec<String>> {
        let cables: Vec<Vec<String>> = read_input_as_rows_strings("advent03/input.txt")
            .iter()
            .map(|x| x.split(',').map(|x| x.to_owned()).collect())
            .collect();
        cables
    }

    fn solve_for(&self, cable1: Vec<String>, cable2: Vec<String>) -> (i32, i32) {
        let cable1 = self.parse_road(cable1);
        let cable2 = self.parse_road(cable2);
        let field1 = self.walk(cable1);
//        println!("Field1={:?}", field1);
        let field2 = self.walk(cable2);
        let mut best_match_distance: Option<i32> = None;
        let mut best_match_latency: Option<i32> = None;
        for (cell1, _) in field1 {
            if let Some(latency2) = field2.get(&cell1) {
                best_match_distance = if_smaller(best_match_distance, cell1.distance());
                best_match_latency = if_smaller(best_match_latency, cell1.latency() + *latency2);
            }
        }
        (best_match_distance.unwrap(), best_match_latency.unwrap())
    }

    fn walk(&self, cable: Vec<(i32, i32)>) -> HashMap<Cell, i32> {
        let mut x = 0;
        let mut y = 0;
        let mut latency = 0;
        let mut field: HashMap<Cell, i32> = HashMap::new();
        for step in cable {
            let (x_step, y_step) = (step.0.signum(), step.1.signum());
            (1..=step.0.abs()).for_each(|_| {
                x += x_step;
                latency += 1;
                Solver::add_cell_if_missing(x, y, latency, &mut field);
            });
            (1..=step.1.abs()).for_each(|_|{
                y += y_step;
                latency += 1;
                Solver::add_cell_if_missing(x, y, latency, &mut field);
            });
        }
        field.remove(&Cell::new(0, 0, 0));
        field
    }

    fn add_cell_if_missing(x: i32, y: i32, latency: i32, field: &mut HashMap<Cell, i32>) -> () {
        let cell = Cell::new(x, y, latency);
        if !field.contains_key(&cell) {
            field.insert(cell, latency);
        }
    }

    fn parse_road(&self, cable: Vec<String>) -> Vec<(i32, i32)> {
        cable.iter().map(|x| self.parse_step(x)).collect()
    }

    fn parse_step(&self, step: &str) -> (i32, i32) {
        match step.chars().next().unwrap() {
            'R' => (step[1..].parse::<i32>().unwrap(), 0),
            'L' => (-step[1..].parse::<i32>().unwrap(), 0),
            'U' => (0, step[1..].parse::<i32>().unwrap()),
            'D' => (0, -step[1..].parse::<i32>().unwrap()),
            _ => panic!("Invalid character encountered!"),
        }
    }
}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 03"
    }

    fn solve_all(&self) -> (String, String) {
        let cables = self.get_data();
        let result = self.solve_for(cables[0].clone(), cables[1].clone());
        (result.0.to_string(), result.1.to_string())
    }

    fn solve_a(&self) -> String {
        unimplemented!()
    }

    fn solve_b(&self) -> String {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::Solver;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test0() {
        assert_eq!(
            Solver {}.solve_for(vec_of_strings!["R8", "U5", "L5", "D3"], vec_of_strings!["U7", "R6", "D4", "L4"]),
            (6, 30)
        );
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(
                vec_of_strings!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                vec_of_strings!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
            ),
            (159, 610)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}.solve_for(
                vec_of_strings!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                vec_of_strings!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
            ),
            (135, 410)
        );
    }
}
