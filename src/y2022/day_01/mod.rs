use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let elves = parse_elves(input);
    Box::new(*elves.iter().max().unwrap())
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut elves = parse_elves(input);
    elves.sort_by(|a, b| b.cmp(a));
    Box::new(elves[0] + elves[1] + elves[2])
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let mut elves = parse_elves(input);
    elves.sort_by(|a, b| b.cmp(a));

    let silver_result = elves[0];
    let gold_result = elves[0] + elves[1] + elves[2];

    (Box::new(silver_result), Box::new(gold_result))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2022, 1, silver, gold)
        .with_both(both)
}

fn parse_elves(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect()
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
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(silver(input).to_string(), "24000");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2022, 1);
        assert_eq!(silver(&input).to_string(), "69310");
    }

    #[test]
    fn gold_test_input() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(gold(input).to_string(), "45000");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2022, 1);
        assert_eq!(gold(&input).to_string(), "206104");
    }
}
