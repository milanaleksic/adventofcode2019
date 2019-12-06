use std::collections::HashMap;

use crate::common::read_input_as_rows_strings;

pub struct Solver {}

impl Solver {
    fn solve(&self, input: Vec<String>) -> String {
        let mut field: HashMap<String, Vec<String>> = HashMap::new();
        input.iter().for_each(|line| {
            let mut splits = line.split(')');
            let split0 = splits.next().unwrap().to_string();
            let split1 = splits.next().unwrap().to_string();
            match field.get(&split1) {
                Some(existing) => {
                    let mut new_vec = existing.clone();
                    new_vec.push(split0);
                    field.insert(split1, new_vec);
                }
                None => {
                    field.insert(split1, vec![split0]);
                }
            }
        });
        let mut count = 0;
        field.iter().for_each(|(_target, sources)| {
            let mut stack = sources.clone();
//            println!("Starting from {:?}", target);
            loop {
//                println!("Current stack {:?}", stack);
                if stack.len() == 0 {
                    break;
                }
                match stack.pop() {
                    Some(existing) => {
                        count += 1;
                        match field.get(existing.as_str()) {
                            Some(children) => {
                                children.iter().for_each(|x|stack.push(x.clone()));
                            }
                            None => (),
                        }
                    }
                    None => count +=1,
                }

            }
        });
        count.to_string()
    }
}

impl crate::Solver for Solver {
    fn name(&self) -> &str {
        "advent 06"
    }

    fn solve_a(&self) -> String {
        let input = read_input_as_rows_strings("advent06/input.txt");
        self.solve(input).to_string()
    }

    fn solve_b(&self) -> String {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solver {}.solve(vec_of_strings!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G",
                                 "G)H", "D)I", "E)J", "J)K", "K)L"]),
            "42"
        );
    }
}
