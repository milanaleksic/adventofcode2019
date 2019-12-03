use super::common;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<usize>, noun: usize, verb: usize) -> String {
        unimplemented!("Later on!")
    }

    fn solve_for(&self, cable1: Vec<&str>, cable2: Vec<&str>) -> i32 {
        return 0;
    }
}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 02"
    }

    fn solve_a(&self) -> String {
        unimplemented!("Later on")
    }

    fn solve_b(&self) -> String {
        unimplemented!("Later on")
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test0() {
        assert_eq!(
            Solver {}.solve_for(vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]),
            6
        );
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(
                vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
            ),
            159
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}.solve_for(
                vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
            ),
            135
        );
    }
}
