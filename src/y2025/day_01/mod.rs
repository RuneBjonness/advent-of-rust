use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut count_zeros = 0;
    let mut position = 50;

    for instruction in input.lines() {
        position = rotate(instruction, position);
        if position == 0 {
            count_zeros += 1;
        }
    }
    Box::new(count_zeros)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut count_zeros = 0;
    let mut position = 50;

    for instruction in input.lines() {
        let result = rotate_and_count_rotations(instruction, position);
        count_zeros += result.rotations;
        position = result.new_position;
    }
    Box::new(count_zeros)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let mut count_zeros_silver = 0;
    let mut count_zeros_gold = 0;
    let mut position = 50;

    for instruction in input.lines() {
        let result = rotate_and_count_rotations(instruction, position);
        count_zeros_gold += result.rotations;
        position = result.new_position;
        if position == 0 {
            count_zeros_silver += 1;
        }
    }
    (Box::new(count_zeros_silver), Box::new(count_zeros_gold))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 1, silver, gold).with_both(both)
}

fn rotate(instruction: &str, from_position: i32) -> i32 {
    let dir = instruction.chars().next().unwrap();
    let length: i32 = instruction[1..].parse().unwrap();

    if dir == 'L' {
        (from_position - length).rem_euclid(100)
    } else {
        (from_position + length).rem_euclid(100)
    }
}

struct RotationResult {
    new_position: i32,
    rotations: i32,
}

fn rotate_and_count_rotations(instruction: &str, from_position: i32) -> RotationResult {
    let dir = instruction.chars().next().unwrap();
    let length: i32 = instruction[1..].parse().unwrap();
    let mut rotations = length / 100;

    if dir == 'L' {
        let mut new_position = (from_position - length) % 100;
        if new_position < 0 {
            new_position += 100;
        }
        if new_position == 0 || (new_position > from_position && from_position != 0) {
            rotations += 1;
        }
        RotationResult {
            new_position,
            rotations,
        }
    } else {
        let mut new_position = (from_position + length) % 100;
        if new_position >= 100 {
            new_position -= 100;
        }
        if new_position == 0 || new_position < from_position {
            rotations += 1;
        }
        RotationResult {
            new_position,
            rotations,
        }
    }
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

    const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "3");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 1);
        assert_eq!(silver(&input).to_string(), "999");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "6");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 1);
        assert_eq!(gold(&input).to_string(), "6099");
    }
}
