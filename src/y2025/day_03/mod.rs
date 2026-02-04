use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let result: i64 = input
        .lines()
        .map(|bank| max_joltage_rating(bank))
        .sum();
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let result: i64 = input
        .lines()
        .map(|bank| {
            let batteries: Vec<u8> = bank
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            max_joltage_rating_remaining(&batteries, 12, String::new())
                .parse::<i64>()
                .unwrap()
        })
        .sum();
    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 3, silver, gold)
}

fn max_joltage_rating(bank: &str) -> i64 {
    let batteries: Vec<u8> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    for i in (1..=9).rev() {
        if let Some(first_index) = batteries.iter().position(|&x| x == i) {
            if first_index + 1 < batteries.len() {
                let max_after = batteries[first_index + 1..]
                    .iter()
                    .max()
                    .copied()
                    .unwrap();
                return (i as i64) * 10 + (max_after as i64);
            }
        }
    }
    0
}

fn max_joltage_rating_remaining(batteries: &[u8], remaining: usize, ratings: String) -> String {
    if remaining == 1 {
        let max_val = batteries.iter().max().copied().unwrap_or(0);
        return ratings + &max_val.to_string();
    }

    for i in (1..=9).rev() {
        if let Some(idx) = batteries.iter().position(|&x| x == i) {
            // Check: need at least (remaining - 1) elements after idx
            if idx + remaining <= batteries.len() {
                return max_joltage_rating_remaining(
                    &batteries[idx + 1..],
                    remaining - 1,
                    ratings + &i.to_string(),
                );
            }
        }
    }
    ratings
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

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "357");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 3);
        assert_eq!(silver(&input).to_string(), "16842");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "3121910778619");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 3);
        assert_eq!(gold(&input).to_string(), "167523425665348");
    }
}
