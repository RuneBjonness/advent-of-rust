use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let result: i32 = input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    });
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            return Box::new(i + 1);
        }
    }
    Box::new(0)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2015, 1, silver, gold)
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
        assert_eq!(silver(")())())").to_string(), "-3");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2015, 1);
        assert_eq!(silver(&input).to_string(), "232");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(")").to_string(), "1");
        assert_eq!(gold("()())").to_string(), "5");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2015, 1);
        assert_eq!(gold(&input).to_string(), "1783");
    }
}
