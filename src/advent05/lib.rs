use std::fmt::{Debug, Error, Formatter};

use crate::common::read_input_as_csv;

enum Instruction {
    ADD(i32),
    MUL(i32),
    WRITE,
    READ(i32),
    JIT(i32),
    JIF(i32),
    LT(i32),
    EQ(i32),
    INT,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Instruction::ADD(instruction) => write!(f, "ADD({})", instruction),
            Instruction::MUL(instruction) => write!(f, "MUL({})", instruction),
            Instruction::WRITE => write!(f, "WRITE"),
            Instruction::READ(instruction) => write!(f, "READ({})", instruction),
            Instruction::JIT(instruction) => write!(f, "JIT({})", instruction),
            Instruction::JIF(instruction) => write!(f, "JIF({})", instruction),
            Instruction::LT(instruction) => write!(f, "LT({})", instruction),
            Instruction::EQ(instruction) => write!(f, "EQ({})", instruction),
            Instruction::INT => write!(f, "INT"),
        }
    }
}

impl Instruction {
    fn exec(&self, proc: &mut Processor) {
        match *self {
            Instruction::ADD(instruction) => {
                let loc = proc.store(instruction, 3);
                proc.input[loc] = proc.operand(instruction, 1) + proc.operand(instruction, 2);
                proc.pc += 4;
            }
            Instruction::MUL(instruction) => {
                let loc = proc.store(instruction, 3);
                proc.input[loc] = proc.operand(instruction, 1) * proc.operand(instruction, 2);
                proc.pc += 4;
            }
            Instruction::WRITE => {
                let target = proc.input[proc.pc + 1];
                proc.input[target as usize] = proc.register;
                proc.pc += 2;
            }
            Instruction::READ(instruction) => {
                proc.register = proc.operand(instruction, 1);
                proc.pc += 2;
            }
            Instruction::JIT(instruction) => {
                if proc.operand(instruction, 1) != 0 {
                    proc.pc = proc.operand(instruction, 2) as usize;
                } else {
                    proc.pc += 3;
                }
            }
            Instruction::JIF(instruction) => {
                if proc.operand(instruction, 1) == 0 {
                    proc.pc = proc.operand(instruction, 2) as usize;
                } else {
                    proc.pc += 3;
                }
            }
            Instruction::LT(instruction) => {
                let loc = proc.store(instruction, 3);
                proc.input[loc] = if proc.operand(instruction, 1) < proc.operand(instruction, 2) {
                    1
                } else {
                    0
                };
                proc.pc += 4;
            }
            Instruction::EQ(instruction) => {
                let loc = proc.store(instruction, 3);
                proc.input[loc] = if proc.operand(instruction, 1) == proc.operand(instruction, 2) {
                    1
                } else {
                    0
                };
                proc.pc += 4;
            }
            Instruction::INT => {
                proc.interrupted = true;
            }
        }
    }

    fn from_bytes(instruction: i32) -> Instruction {
        match instruction % 100 {
            1 => Instruction::ADD(instruction),
            2 => Instruction::MUL(instruction),
            3 => Instruction::WRITE,
            4 => Instruction::READ(instruction),
            5 => Instruction::JIT(instruction),
            6 => Instruction::JIF(instruction),
            7 => Instruction::LT(instruction),
            8 => Instruction::EQ(instruction),
            99 => Instruction::INT,
            x => panic!("Not expected value {:?}", x),
        }
    }
}

pub struct Processor {
    input: Vec<i32>,
    register: i32,
    pc: usize,
    interrupted: bool,
}

impl Processor {
    fn new(input: Vec<i32>, register: i32) -> Processor {
        Processor {
            input,
            register,
            pc: 0,
            interrupted: false,
        }
    }

    fn running(&self) -> bool {
        !self.interrupted
    }

    fn operand(&self, instruction: i32, op: usize) -> i32 {
        let val_or_adr = self.input[self.pc + op];
        match self.op_mode(instruction, op) {
            0 => self.input[val_or_adr as usize],
            1 => val_or_adr,
            x => panic!("Unexpected input {:?}", x),
        }
    }

    fn store(&self, instruction: i32, op: usize) -> usize {
        match self.op_mode(instruction, op) {
            0 => self.input[self.pc + op] as usize,
            1 => self.pc + op,
            x => panic!("Unexpected input {:?}", x),
        }
    }

    fn step(&mut self) {
        let instruction = self.next();
//        println!(
//            "PC={} register={} instruction={:?}",
//            self.pc, self.register, instruction
//        );
        instruction.exec(self);
    }

    fn next(&self) -> Instruction {
        let instruction = *self.input.get(self.pc).unwrap();
        Instruction::from_bytes(instruction)
    }

    pub fn state(&self) -> (Vec<i32>, i32) {
        (self.input.clone(), self.register)
    }

    fn op_mode(&self, instruction: i32, op: usize) -> usize {
        let field = 10_i32.pow((op - 1) as u32);
        (instruction / 100 / field % 10) as usize
    }
}

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<i32>, register: i32) -> String {
        self.solve_for(input.clone(), register).1.to_string()
    }

    fn solve_for(&self, input: Vec<i32>, register: i32) -> (Vec<i32>, i32) {
        let mut processor = Processor::new(input, register);
        while processor.running() {
            processor.step()
        }
        return processor.state();
    }
}

impl crate::Solver for Solver {
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

    const BIGGER_EXAMPLE: &'static [i32] = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    #[test]
    fn test4() {
        assert_eq!(Solver {}.solve_for(BIGGER_EXAMPLE.to_vec(), 7).1, 999);
    }

    #[test]
    fn test5() {
        assert_eq!(Solver {}.solve_for(BIGGER_EXAMPLE.to_vec(), 8).1, 1000);
    }

    #[test]
    fn test6() {
        assert_eq!(Solver {}.solve_for(BIGGER_EXAMPLE.to_vec(), 10).1, 1001);
    }
}
