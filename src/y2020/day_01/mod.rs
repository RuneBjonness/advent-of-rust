use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    numbers.sort_unstable();

    for i in 0..numbers.len() {
        for j in (i + 1..numbers.len()).rev() {
            let sum = numbers[i] + numbers[j];
            if sum == 2020 {
                return Box::new(numbers[i] * numbers[j]);
            } else if sum < 2020 {
                break;
            }
        }
    }
    Box::new(0)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    numbers.sort_unstable();

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let partial_sum = numbers[i] + numbers[j];
            for k in (i + 1..numbers.len()).rev() {
                let sum = partial_sum + numbers[k];
                if sum == 2020 {
                    return Box::new(numbers[i] * numbers[j] * numbers[k]);
                } else if sum < 2020 {
                    break;
                }
            }
        }
    }
    Box::new(0)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2020, 1, silver, gold)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(year: u16, day: u8) -> String {
        fs::read_to_string(format!("./input/{}_{:02}.txt", year, day))
            .unwrap()
            .trim_end()
            .to_string()
    }

    #[test]
    fn silver_test_input() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(silver(input).to_string(), "514579");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2020, 1);
        assert_eq!(silver(&input).to_string(), "918339");
    }

    #[test]
    fn gold_test_input() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(gold(input).to_string(), "241861950");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2020, 1);
        assert_eq!(gold(&input).to_string(), "23869440");
    }
}
