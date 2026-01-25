use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut sum = 0;
    let chars: Vec<char> = input.chars().collect();

    if chars.is_empty() {
        return Box::new(sum);
    }

    let first_char = chars[0];
    let mut value = first_char;

    // Create circular sequence by appending first char
    for &next_value in chars.iter().skip(1) {
        if next_value == value {
            sum += value.to_digit(10).unwrap() as i32;
        }
        value = next_value;
    }

    // Check if last char matches first char (circular)
    if value == first_char {
        sum += value.to_digit(10).unwrap() as i32;
    }

    Box::new(sum)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut sum = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let half = len / 2;

    for i in 0..len {
        let j = (i + half) % len;
        if chars[i] == chars[j] {
            sum += chars[i].to_digit(10).unwrap() as i32;
        }
    }

    Box::new(sum)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2017, 1, silver, gold)
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
        assert_eq!(silver("1122").to_string(), "3");
        assert_eq!(silver("1111").to_string(), "4");
        assert_eq!(silver("1234").to_string(), "0");
        assert_eq!(silver("91212129").to_string(), "9");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2017, 1);
        assert_eq!(silver(&input).to_string(), "1097");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("1212").to_string(), "6");
        assert_eq!(gold("1221").to_string(), "0");
        assert_eq!(gold("123425").to_string(), "4");
        assert_eq!(gold("123123").to_string(), "12");
        assert_eq!(gold("12131415").to_string(), "4");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2017, 1);
        assert_eq!(gold(&input).to_string(), "1188");
    }
}
