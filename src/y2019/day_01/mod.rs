use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

fn required_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn additional_required_fuel(fuel: i32) -> i32 {
    let mut total = 0;
    let mut fuel = fuel;
    while fuel > 8 {
        fuel = required_fuel(fuel);
        total += fuel;
    }
    total
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let result: i32 = input
        .lines()
        .map(|x| required_fuel(x.parse::<i32>().unwrap()))
        .sum();
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let result: i32 = input
        .lines()
        .map(|x| required_fuel(x.parse::<i32>().unwrap()))
        .map(|x| x + additional_required_fuel(x))
        .sum();
    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2019, 1, silver, gold)
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
        let test_input = "12\n14\n1969\n100756";
        assert_eq!(silver(test_input).to_string(), "34241");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2019, 1);
        assert_eq!(silver(&input).to_string(), "3147032");
    }

    #[test]
    fn gold_test_input() {
        let test_input = "14\n1969\n100756";
        assert_eq!(gold(test_input).to_string(), "51314");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2019, 1);
        assert_eq!(gold(&input).to_string(), "4717699");
    }
}
