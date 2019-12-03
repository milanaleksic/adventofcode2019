use std::collections::HashMap;

use crate::common::read_input_as_rows_strings;

use super::common;

pub struct Solver {}

impl Solver {
    fn solve_for(&self, cable1: Vec<String>, cable2: Vec<String>) -> i32 {
        let cable1 = self.parse_road(cable1);
        let cable2 = self.parse_road(cable2);
        let field1 = self.walk(cable1);
        let field2 = self.walk(cable2);
        let mut best_match: Option<i32> = None;
        for (slot, _) in field1 {
            if let Some(_) = field2.get(&slot) {
                let new = slot.0.abs() + slot.1.abs();
                best_match = match best_match {
                    Some(m) => {
                        if new < m {
                            Some(new)
                        } else {
                            best_match
                        }
                    }
                    None => Some(new),
                };
            }
        }
        best_match.unwrap()
    }

    fn walk(&self, cable: Vec<(i32, i32)>) -> HashMap<(i32, i32), bool> {
        let mut iter = (0, 0);
        let mut field: HashMap<(i32, i32), bool> = HashMap::new();
        for step in cable {
            if step.0 != 0 {
                let (begin, end) = if step.0 < 0 { (step.0, 0) } else { (0, step.0) };
                for ix in begin..end {
                    field.insert((iter.0 + ix, iter.1), true);
                }
                iter.0 += step.0;
            }
            if step.1 != 0 {
                let (begin, end) = if step.1 < 0 { (step.1, 0) } else { (0, step.1) };
                for iy in begin..end {
                    field.insert((iter.0, iter.1 + iy), true);
                }
                iter.1 += step.1;
            }
        }
        field.remove(&(0, 0));
        field
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

    fn solve_a(&self) -> String {
        let cables: Vec<Vec<String>> = read_input_as_rows_strings("advent03/input.txt")
            .iter()
            .map(|x| x.split(',').map(|x| x.to_owned()).collect())
            .collect();
        format!("{}", self.solve_for(cables[0].clone(), cables[1].clone()))
    }

    fn solve_b(&self) -> String {
        unimplemented!("Later on")
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
            6
        );
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(
                vec_of_strings!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                vec_of_strings!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
            ),
            159
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}.solve_for(
                vec_of_strings!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                vec_of_strings!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
            ),
            135
        );
    }
}
