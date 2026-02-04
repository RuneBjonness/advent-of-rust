use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let (mut list1, mut list2) = parse_lists(input);
    list1.sort_unstable();
    list2.sort_unstable();

    let result: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let (list1, list2) = parse_lists(input);

    let result: i32 = list1
        .iter()
        .map(|&x| x * count_occurrences(&list2, x))
        .sum();

    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2024, 1, silver, gold)
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            (a, b)
        })
        .unzip()
}

fn count_occurrences(list: &[i32], value: i32) -> i32 {
    list.iter().filter(|&&n| n == value).count() as i32
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(silver(input).to_string(), "11");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2024, 1);
        assert_eq!(silver(&input).to_string(), "2430334");
    }

    #[test]
    fn gold_test_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(gold(input).to_string(), "31");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2024, 1);
        assert_eq!(gold(&input).to_string(), "28786472");
    }
}
