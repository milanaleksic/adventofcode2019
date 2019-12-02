use crate::common::read_input_as_csv;

use super::common;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<usize>, noun: usize, verb: usize) -> String {
        let mut input2 = input.clone();
        input2[1] = noun;
        input2[2] = verb;
        self.solve_for(input2)[0].to_string()
    }

    fn solve_for(&self, mut input: Vec<usize>) -> Vec<usize> {
        let mut pc = 0;
        loop {
            match input.get(pc) {
                Some(1) => {
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    input[target] = input[op1] + input[op2];
                    pc += 4;
                }
                Some(2) => {
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    input[target] = input[op1] * input[op2];
                    pc += 4;
                }
                Some(99) => {
                    break;
                }
                x => panic!("Not expected value {:?}", x),
            }
        }
        return input;
    }
}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 02"
    }

    fn solve_a(&self) -> String {
        let input = read_input_as_csv::<usize>("advent02/input.txt");
        self.solve(&input, 12, 2).to_string()
    }

    fn solve_b(&self) -> String {
        let expected_value = String::from("19690720");
        let input = read_input_as_csv::<usize>("advent02/input.txt");
        for i in 0..99 {
            for j in 0..99 {
                if self.solve(&input, i, j) == expected_value {
                    return format!("noun={}, verb={}, answer={}", i, j, 100 * i + j);
                }
            }
        }
        panic!("Not reached an answer!")
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(vec![1, 0, 0, 0, 99]),
            vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}.solve_for(vec![2, 3, 0, 3, 99]),
            vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solver {}.solve_for(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solver {}.solve_for(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
