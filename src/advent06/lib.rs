use std::collections::HashMap;

use crate::common::read_input_as_rows_strings;

pub struct Solver {}

impl Solver {
    fn make_graph(&self, input: &Vec<String>) -> HashMap<String, String> {
        let mut field: HashMap<String, String> = HashMap::new();
        input.iter().for_each(|line| {
            let mut splits = line.split(')');
            let split0 = splits.next().unwrap().to_string();
            let split1 = splits.next().unwrap().to_string();
            field.insert(split1, split0);
        });
        field
    }

    fn solve_graph_paths(&self, input: Vec<String>) -> String {
        let field = self.make_graph(&input);
        let mut count = 0;
        field.iter().for_each(|(_target, source)| {
            let mut iter = Some(source);
            while let Some(x) = iter {
                count += 1;
                iter = field.get(x);
            }
        });
        count.to_string()
    }

    fn solve_common_prefix(&self, input: Vec<String>) -> String {
        let path1 = self.get_orbits(&input, "SAN");
        let path2 = self.get_orbits(&input, "YOU");
        let mut common_parent = 0;
        while path1.get(common_parent) == path2.get(common_parent)
            && common_parent < path1.len()
            && common_parent < path2.len()
        {
            common_parent += 1;
        }
        ((path1.len() - common_parent) + (path2.len() - common_parent)).to_string()
    }

    fn get_orbits(&self, input: &Vec<String>, planet: &str) -> Vec<String> {
        let field = self.make_graph(&input);
        let mut path = vec![];
        let mut iter = field.get(planet);
        while let Some(x) = iter {
            path.insert(0, x.clone());
            iter = field.get(x);
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
