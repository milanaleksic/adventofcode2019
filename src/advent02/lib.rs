use super::common;
use crate::common::read_input_as_csv_ints;

pub struct Solver {}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 02"
    }

    fn solve_a(&self) -> String {
        let mut input = read_input_as_csv_ints("advent02/input.txt");
        input[1] = 12;
        input[2] = 2;
        solve01(input)[0].to_string()
    }

    fn solve_b(&self) -> String {
        let input = read_input_as_csv_ints("advent02/input.txt");
        for i in 0..99 {
            for j in 0..99 {
                let mut input2 = input.clone();
                input2[1] = i;
                input2[2] = j;
                if solve01(input2)[0] == 19690720 {
                    return format!("noun={}, verb={}, answer={}", i, j, 100*i+j);
                }
            }
        }
        panic!("Not reached an answer!")
    }
}

fn solve01(input: Vec<usize>) -> Vec<usize> {
    let mut output = input.clone();
    let mut pc = 0;
    loop {
        match output.get(pc) {
            Some(1) => {
                let target = output[pc + 3];
                let op1 = output[pc + 1];
                let op2 = output[pc + 2];
                output[target] = output[op1] + output[op2];
                pc += 4;
            }
            Some(2) => {
                let target = output[pc + 3];
                let op1 = output[pc + 1];
                let op2 = output[pc + 2];
                output[target] = output[op1] * output[op2];
                pc += 4;
            }
            Some(99) => {
                break;
            }
            x => panic!("Not expected value {:?}", x)
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::solve01;

    #[test]
    fn test1() {
        assert_eq!(solve01(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test2() {
        assert_eq!(solve01(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test3() {
        assert_eq!(solve01(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test4() {
        assert_eq!(solve01(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
