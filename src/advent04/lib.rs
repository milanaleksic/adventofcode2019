use crate::common;

pub struct Solver {}

impl Solver {
    fn has_two_identical_numbers_and_increasing(&self, num: i32) -> bool {
        let mut previous = None;
        let mut num = num;
        let mut equal = false;
        for _ in 1..=6 {
            let last = num % 10;
            if let Some(p) = previous {
                if p == last {
                    equal = true;
                }
                if p < last {
                    return false;
                }
            }
            previous = Some(last);
            num = num / 10;
        }
        return equal;
    }

}

impl common::Solver for Solver {
    fn name(&self) -> &str {
        "advent 04"
    }

    fn solve_a(&self) -> String {
        let mut count = 0;
        for i in 240920..=789857 {
            if self.has_two_identical_numbers_and_increasing(i) {
                count+=1;
            }
        }
        return count.to_string();
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
        assert_eq!(Solver {}.has_two_identical_numbers_and_increasing(111111), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solver {}.has_two_identical_numbers_and_increasing(223450), false);
    }

    #[test]
    fn test3() {
        assert_eq!(Solver {}.has_two_identical_numbers_and_increasing(123789), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solver {}.has_two_identical_numbers_and_increasing(240955), false);
    }

}
