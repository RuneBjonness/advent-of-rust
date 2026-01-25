use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    Box::new(increase_count(&numbers, 1))
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    Box::new(increase_count(&numbers, 3))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 1, silver, gold)
}

fn increase_count(series: &[i32], chunk_size: usize) -> usize {
    let mut count = 0;
    for i in chunk_size..series.len() {
        if series[i] > series[i - chunk_size] {
            count += 1;
        }
    }
    count
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
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(silver(input).to_string(), "7");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 1);
        assert_eq!(silver(&input).to_string(), "1228");
    }

    #[test]
    fn gold_test_input() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(gold(input).to_string(), "5");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 1);
        assert_eq!(gold(&input).to_string(), "1257");
    }
}
