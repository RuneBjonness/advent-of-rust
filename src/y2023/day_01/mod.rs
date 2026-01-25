use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let result: u32 = input
        .lines()
        .map(|line| first_digit(line) * 10 + last_digit(line))
        .sum();
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let result: u32 = input
        .lines()
        .map(|line| first_digit_or_spelled(line) * 10 + last_digit_or_spelled(line))
        .sum();
    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2023, 1, silver, gold)
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit(txt: &str) -> u32 {
    txt.chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0)
}

fn last_digit(txt: &str) -> u32 {
    txt.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0)
}

fn first_digit_or_spelled(txt: &str) -> u32 {
    let mut min_pos = txt.len();
    let mut digit = 0;

    // Check for numeric digits
    if let Some(pos) = txt.find(|c: char| c.is_ascii_digit()) {
        min_pos = pos;
        digit = txt.chars().nth(pos).unwrap().to_digit(10).unwrap();
    }

    // Check for spelled-out digits
    for (i, spelled) in DIGITS.iter().enumerate() {
        if let Some(pos) = txt.find(spelled) {
            if pos < min_pos {
                min_pos = pos;
                digit = i as u32;
            }
        }
    }

    digit
}

fn last_digit_or_spelled(txt: &str) -> u32 {
    let mut max_pos = None;
    let mut digit = 0;

    // Check for numeric digits
    for (pos, c) in txt.char_indices() {
        if c.is_ascii_digit() {
            max_pos = Some(pos);
            digit = c.to_digit(10).unwrap();
        }
    }

    // Check for spelled-out digits
    for (i, spelled) in DIGITS.iter().enumerate() {
        if let Some(pos) = txt.rfind(spelled) {
            if max_pos.is_none() || pos > max_pos.unwrap() {
                max_pos = Some(pos);
                digit = i as u32;
            }
        }
    }

    digit
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
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(silver(input).to_string(), "142");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2023, 1);
        assert_eq!(silver(&input).to_string(), "54877");
    }

    #[test]
    fn gold_test_input() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(gold(input).to_string(), "281");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2023, 1);
        assert_eq!(gold(&input).to_string(), "54100");
    }
}
