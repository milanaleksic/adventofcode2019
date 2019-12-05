use crate::common::read_input_as_csv;

use super::common;

const ADD: i32 = 1;
const MUL: i32 = 2;
const WRITE: i32 = 3;
const READ: i32 = 4;
const JIT: i32 = 5;
const JIF: i32 = 6;
const LT: i32 = 7;
const EQ: i32 = 8;
const INT: i32 = 99;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<i32>, data: i32) -> String {
        self.solve_for(input.clone(), data).1.to_string()
    }

    fn op_mode(&self, instruction: i32, op: usize) -> usize {
        let field = 10_i32.pow((op - 1) as u32);
        (instruction / 100 / field % 10) as usize
    }

    fn solve_for(&self, mut input: Vec<i32>, data: i32) -> (Vec<i32>, i32) {
        let mut data = data;
        let mut pc = 0;
        loop {
            let instruction = *input.get(pc).unwrap();
            let operand = |op| {
                let val_or_adr = input[pc + op];
                match self.op_mode(instruction, op) {
                    0 => input[val_or_adr as usize],
                    1 => val_or_adr,
                    _ => panic!("Unexpected input"),
                }
            };
            let store = |op| match self.op_mode(instruction, op) {
                0 => input[pc + op] as usize,
                1 => pc + op,
                _ => panic!("Unexpected input"),
            };
            match instruction % 100 {
                ADD => {
                    let loc = store(3);
                    input[loc] = operand(1) + operand(2);
                    pc += 4;
                }
                MUL => {
                    let loc = store(3);
                    input[loc] = operand(1) * operand(2);
                    pc += 4;
                }
                WRITE => {
                    let target = input[pc + 1];
                    input[target as usize] = data;
                    pc += 2;
                }
                READ => {
                    data = operand(1);
                    pc += 2;
                }
                JIT => {
                    if operand(1) != 0 {
                        pc = operand(2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                JIF => {
                    if operand(1) == 0 {
                        pc = operand(2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                LT => {
                    let loc = store(3);
                    input[loc] = if operand(1) < operand(2) { 1 } else { 0 };
                    pc += 4;
                }
                EQ => {
                    let loc = store(3);
                    input[loc] = if operand(1) == operand(2) { 1 } else { 0 };
                    pc += 4;
                }
                INT => {
                    break;
                }
                x => panic!("Not expected value {:?}", x),
            }
        }
        return (input, data);
    }
}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 05"
    }

    fn solve_a(&self) -> String {
        let input = read_input_as_csv::<i32>("advent05/input.txt");
        self.solve(&input, 1).to_string()
    }

    fn solve_b(&self) -> String {
        let input = read_input_as_csv::<i32>("advent05/input.txt");
        self.solve(&input, 5).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve_for(vec![1002, 4, 3, 4, 33], 1),
            (vec![1002, 4, 3, 4, 99], 1)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}
                .solve_for(
                    vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                    0,
                )
                .1,
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solver {}
                .solve_for(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0)
                .1,
            0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solver {}
                .solve_for(
                    vec![
                        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106,
                        0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1,
                        46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                    ],
                    7,
                )
                .1,
            999
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solver {}
                .solve_for(
                    vec![
                        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106,
                        0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1,
                        46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                    ],
                    8,
                )
                .1,
            1000
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solver {}
                .solve_for(
                    vec![
                        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106,
                        0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1,
                        46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                    ],
                    10,
                )
                .1,
            1001
        );
    }
}
