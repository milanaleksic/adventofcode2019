use super::common;
use crate::common::read_input_as_rows;

pub struct Solver {}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 01"
    }

    fn solve_a(&self) -> String {
        read_input_as_rows::<i32>("advent01/input.txt")
            .iter()
            .map(|x| Fuel::fuel(x.clone()))
            .sum::<i32>()
            .to_string()
    }

    fn solve_b(&self) -> String {
        read_input_as_rows::<i32>("advent01/input2.txt")
            .iter()
            .map(|x| fuel2(x.clone()))
            .sum::<i32>()
            .to_string()
    }
}

pub struct Fuel {
    pub value: i32,
}

impl Fuel {
    pub fn from(fuel: i32) -> Fuel {
        Fuel {
            value: fuel.clone(),
        }
    }
    pub fn fuel(f: i32) -> i32 {
        f / 3 - 2
    }
}

impl Iterator for Fuel {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.value = Fuel::fuel(self.value);
        if self.value >= 0 {
            Some(self.value)
        } else {
            None
        }
    }
}

pub fn fuel2(f: i32) -> i32 {
    Fuel::from(f).sum()
}

#[cfg(test)]
mod tests {
    use super::{fuel2, Fuel};

    #[test]
    fn test12() {
        assert_eq!(Fuel::fuel(12), 2);
    }

    #[test]
    fn test14() {
        assert_eq!(Fuel::fuel(14), 2);
    }

    #[test]
    fn test1969() {
        assert_eq!(Fuel::fuel(1969), 654);
    }

    #[test]
    fn test100756() {
        assert_eq!(Fuel::fuel(100756), 33583);
    }

    #[test]
    fn test2_14() {
        assert_eq!(fuel2(12), 2);
    }

    #[test]
    fn test2_1969() {
        assert_eq!(fuel2(1969), 966);
    }

    #[test]
    fn test2_50346() {
        assert_eq!(fuel2(100756), 50346);
    }
}
