use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut h_pos = 0;
    let mut v_pos = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => h_pos += dist,
            "up" => v_pos -= dist,
            "down" => v_pos += dist,
            _ => {}
        }
    }

    Box::new(h_pos * v_pos)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut h_pos = 0;
    let mut v_pos = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let val = parts.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => {
                h_pos += val;
                v_pos += aim * val;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => {}
        }
    }

    Box::new(h_pos * v_pos)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 2, silver, gold)
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
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(silver(input).to_string(), "150");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 2);
        assert_eq!(silver(&input).to_string(), "2272262");
    }

    #[test]
    fn gold_test_input() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(gold(input).to_string(), "900");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 2);
        assert_eq!(gold(&input).to_string(), "2134882034");
    }
}
