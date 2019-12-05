use crate::common::read_input_as_csv;

use super::common;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<i32>, noun: i32, verb: i32) -> String {
        let mut input2 = input.clone();
        input2[1] = noun;
        input2[2] = verb;
        self.solve_for(input2).to_string()
    }

    fn solve_for(&self, mut input: Vec<i32>) -> i32 {
        let mut data = 1;
        let mut pc = 0;
        loop {
            let instruction = input.get(pc).unwrap();
            let opcode = instruction % 100;
            let op1mode = instruction / 100 % 10;
            let op2mode = instruction / 1000 % 10;
            let op3mode = instruction / 10000 % 10;
            match opcode {
                1 => {
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let result = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    } + match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    match op3mode {
                        0 => input[target as usize] = result,
                        1 => input[pc+3] = result,
                        _ => panic!("Unexpected input"),
                    }
                    pc += 4;
                }
                2 => {
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let result = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    } * match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    match op3mode {
                        0 => input[target as usize] = result,
                        1 => input[pc+3] = result,
                        _ => panic!("Unexpected input"),
                    }
                    pc += 4;
                }
                3 => {
                    let target = input[pc + 1];
                    input[target as usize] = data;
                    pc += 2;
                }
                4 => {
                    let target = input[pc + 1];
                    data = input[target as usize];
                    pc += 2;
                }
                99 => {
                    break;
                }
                x => panic!("Not expected value {:?}", x),
            }
        }
        return data;
    }
}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 02"
    }

    fn solve_a(&self) -> String {
        let input = read_input_as_csv::<i32>("advent05/input.txt");
        self.solve(&input, 12, 2).to_string()
    }

    fn solve_b(&self) -> String {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(vec![1002,4,3,4]),
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
