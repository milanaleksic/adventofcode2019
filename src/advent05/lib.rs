use crate::common::read_input_as_csv;

use super::common;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: &Vec<i32>, data: i32) -> String {
        self.solve_for(input.clone(), data).1.to_string()
    }

    fn solve_for(&self, mut input: Vec<i32>, data: i32) -> (Vec<i32>, i32) {
        let mut data = data;
        let mut pc = 0;
        loop {
            let instruction = input.get(pc).unwrap();
            println!("instruction={} at pc={}", instruction, pc);
            let opcode = instruction % 100;
            let op1mode = instruction / 100 % 10;
            let op2mode = instruction / 1000 % 10;
            let op3mode = instruction / 10000 % 10;
            match opcode {
                1 => {
                    println!("1");
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let val1 = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    };
                    let val2 = match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    let result = val1 + val2;
                    println!("op3mode={} val1={} val2={} result={} input[225]={}", op3mode, val1, val2, result, input[225]);
                    match op3mode {
                        0 => input[target as usize] = result,
                        1 => input[pc+3] = result,
                        _ => panic!("Unexpected input"),
                    }
                    pc += 4;
                }
                2 => {
                    println!("2");
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
                5 => {
                    println!("5");
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let cond = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    };
                    let pointer = match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    if cond != 0 {
                        pc = pointer as usize;
                    } else {
                        pc += 3;
                    }
                }
                6 => {
                    println!("6");
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let cond = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    };
                    let pointer = match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    if cond == 0 {
                        pc = pointer as usize;
                    } else {
                        pc += 3;
                    }
                }
                3 => {
                    println!("3");
                    let target = input[pc + 1];
                    input[target as usize] = data;
                    pc += 2;
                }
                7 => {
                    println!("7");
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let o1 = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    };
                    let o2 = match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    if o1 < o2 {
                        match op3mode {
                            0 => input[target as usize] = 1,
                            1 => input[pc+3] = 1,
                            _ => panic!("Unexpected input"),
                        }
                    } else {
                        match op3mode {
                            0 => input[target as usize] = 0,
                            1 => input[pc+3] = 0,
                            _ => panic!("Unexpected input"),
                        }
                    }
                    pc += 4;
                }
                8 => {
                    println!("8");
                    let target = input[pc + 3];
                    let op1 = input[pc + 1];
                    let op2 = input[pc + 2];
                    let o1 = match op1mode {
                        0 => input[op1 as usize],
                        1 => op1,
                        _ => panic!("Unexpected input"),
                    };
                    let o2 = match op2mode {
                        0 => input[op2 as usize],
                        1 => op2,
                        _ => panic!("Unexpected input"),
                    };
                    if o1 == o2 {
                        match op3mode {
                            0 => input[target as usize] = 1,
                            1 => input[pc+3] = 1,
                            _ => panic!("Unexpected input"),
                        }
                    } else {
                        match op3mode {
                            0 => input[target as usize] = 0,
                            1 => input[pc+3] = 0,
                            _ => panic!("Unexpected input"),
                        }
                    }
                    pc += 4;
                }
                4 => {
                    println!("4");
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
            Solver {}.solve_for(vec![1002,4,3,4,33]),
            (vec![1002,4,3,4,99], 1)
        );
    }

//    #[test]
//    fn test2() {
//        assert_eq!(
//            Solver {}.solve_for(vec![3,225,1,225,6,6,1100]),
//            (vec![3,225,1,225,6,6,1101], 1)
//        );
//    }
}
