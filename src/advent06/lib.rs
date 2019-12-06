use std::collections::HashMap;

use crate::common::read_input_as_rows_strings;

pub struct Solver {}

impl Solver {
    fn make_graph(&self, input: &Vec<String>) -> HashMap<String, Vec<String>> {
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
        field
    }

    fn solve_graph_paths(&self, input: Vec<String>) -> String {
        let field = self.make_graph(&input);
        let mut count = 0;
        field.iter().for_each(|(_target, sources)| {
            let mut stack = sources.clone();
            loop {
                if stack.len() == 0 {
                    break;
                }
                match stack.pop() {
                    Some(existing) => {
                        count += 1;
                        match field.get(existing.as_str()) {
                            Some(children) => {
                                children.iter().for_each(|x| stack.push(x.clone()));
                            }
                            None => (),
                        }
                    }
                    None => count += 1,
                }
            }
        });
        count.to_string()
    }

    fn solve_common_prefix(&self, input: Vec<String>) -> String {
        let path1 = self.get_orbits(&input, "SAN");
        let path2 = self.get_orbits(&input, "YOU");
        println!("SAN={:?}, YOU={:?}", path1, path2);
        let mut common_parent = 0;
        loop {
            if path1.get(common_parent) != path2.get(common_parent)
                || common_parent >= path1.len()
                || common_parent >= path2.len()
            {
                break;
            }
            common_parent += 1;
        }
        println!("common parent={}", common_parent);
        ((path1.len() - common_parent) + (path2.len() - common_parent)).to_string()
    }

    fn get_orbits(&self, input: &Vec<String>, planet: &str) -> Vec<String> {
        let field = self.make_graph(&input);
        let sources = field.get(planet).unwrap();
        let mut path = vec![];
        let mut stack = sources.clone();
        loop {
            if stack.len() == 0 {
                break;
            }
            match stack.pop() {
                Some(existing) => {
                    path.insert(0, existing.clone());
                    match field.get(existing.as_str()) {
                        Some(children) => {
                            children.iter().for_each(|x| stack.push(x.clone()));
                        }
                        None => (),
                    }
                    ()
                }
                None => (),
            }
        }
        path
    }
}

impl crate::Solver for Solver {
    fn name(&self) -> &str {
        "advent 06"
    }

    fn solve_a(&self) -> String {
        let input = read_input_as_rows_strings("advent06/input.txt");
        self.solve_graph_paths(input).to_string()
    }

    fn solve_b(&self) -> String {
        let input = read_input_as_rows_strings("advent06/input.txt");
        self.solve_common_prefix(input).to_string()
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
            Solver {}.solve_graph_paths(vec_of_strings![
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"
            ]),
            "42"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solver {}.solve_common_prefix(vec_of_strings![
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
                "K)YOU", "I)SAN"
            ]),
            "4"
        );
    }
}
