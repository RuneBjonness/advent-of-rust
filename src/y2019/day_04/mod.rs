use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let (min, max) = parse_range(input);
    let count = (min..=max).filter(|&i| is_valid_password(i, false)).count();
    Box::new(count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let (min, max) = parse_range(input);
    let count = (min..=max).filter(|&i| is_valid_password(i, true)).count();
    Box::new(count)
}

fn parse_range(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input.split('-').map(|s| s.parse().unwrap()).collect();
    (parts[0], parts[1])
}

fn is_valid_password(pwd: u32, exact_double: bool) -> bool {
    let digits: Vec<u32> = pwd
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut has_double = false;

    for i in 0..digits.len() {
        // Check if digits never decrease
        if i > 0 && digits[i] < digits[i - 1] {
            return false;
        }

        if exact_double {
            // Part 2: exactly one pair not part of a larger group
            let not_part_of_triple_before = i < 2 || digits[i] != digits[i - 2];
            let matches_previous = i > 0 && digits[i] == digits[i - 1];
            let not_part_of_triple_after = i + 1 == digits.len() || digits[i] != digits[i + 1];

            if not_part_of_triple_before && matches_previous && not_part_of_triple_after {
                has_double = true;
            }
        } else {
            // Part 1: any adjacent matching digits
            if i > 0 && digits[i] == digits[i - 1] {
                has_double = true;
            }
        }
    }

    has_double
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2019, 4, silver, gold)
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
        assert_eq!(silver("111111-111111").to_string(), "1");
        assert_eq!(silver("223450-223450").to_string(), "0");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2019, 4);
        assert_eq!(silver(&input).to_string(), "1150");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("123444-123444").to_string(), "0");
        assert_eq!(gold("111122-111122").to_string(), "1");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2019, 4);
        assert_eq!(gold(&input).to_string(), "748");
    }
}
