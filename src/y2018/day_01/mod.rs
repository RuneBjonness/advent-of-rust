use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashSet;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let result: i32 = input.lines().map(|v| v.parse::<i32>().unwrap()).sum();
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let changes: Vec<i32> = input.lines().map(|v| v.parse::<i32>().unwrap()).collect();
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut frequency = 0;

    loop {
        for &change in &changes {
            frequency += change;
            if seen.contains(&frequency) {
                return Box::new(frequency);
            }
            seen.insert(frequency);
        }
    }
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2018, 1, silver, gold)
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
        assert_eq!(silver("+1\n-2\n+3\n+1").to_string(), "3");
    }

    #[test]
    fn silver_test_input_2() {
        assert_eq!(silver("+1\n+1\n+1").to_string(), "3");
    }

    #[test]
    fn silver_test_input_3() {
        assert_eq!(silver("+1\n+1\n-2").to_string(), "0");
    }

    #[test]
    fn silver_test_input_4() {
        assert_eq!(silver("-1\n-2\n-3").to_string(), "-6");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("+1\n-2\n+3\n+1").to_string(), "2");
    }

    #[test]
    fn gold_test_input_2() {
        assert_eq!(gold("+1\n-1").to_string(), "0");
    }

    #[test]
    fn gold_test_input_3() {
        assert_eq!(gold("+3\n+3\n+4\n-2\n-4").to_string(), "10");
    }

    #[test]
    fn gold_test_input_4() {
        assert_eq!(gold("-6\n+3\n+8\n+5\n-6").to_string(), "5");
    }

    #[test]
    fn gold_test_input_5() {
        assert_eq!(gold("+7\n+7\n-2\n-7\n-4").to_string(), "14");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2018, 1);
        assert_eq!(silver(&input).to_string(), "540");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2018, 1);
        assert_eq!(gold(&input).to_string(), "73056");
    }
}
