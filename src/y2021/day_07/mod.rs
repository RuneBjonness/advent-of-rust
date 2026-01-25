use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashSet;
use std::fmt::Display;

fn triangular(n: i32) -> i32 {
    n * (n + 1) / 2
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let crabs: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let positions: Vec<i32> = {
        let mut unique: Vec<i32> = crabs
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique.sort_unstable();
        unique
    };

    let mut up_idx = positions[0];
    let mut next_cost_up = crabs.iter().filter(|&&x| x == up_idx).count() as i32;

    let mut down_idx = positions[positions.len() - 1];
    let mut next_cost_down = crabs.iter().filter(|&&x| x == down_idx).count() as i32;

    let mut total_cost = 0;

    while up_idx < down_idx {
        if next_cost_up < next_cost_down {
            up_idx += 1;
            total_cost += next_cost_up;
            next_cost_up += crabs.iter().filter(|&&x| x == up_idx).count() as i32;
        } else {
            down_idx -= 1;
            total_cost += next_cost_down;
            next_cost_down += crabs.iter().filter(|&&x| x == down_idx).count() as i32;
        }
    }

    Box::new(total_cost)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let crabs: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let positions: Vec<i32> = {
        let mut unique: Vec<i32> = crabs
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique.sort_unstable();
        unique
    };

    let mut least_cost = i32::MAX;
    for i in positions[0]..positions[positions.len() - 1] {
        let mut cost = 0;
        for &p in &positions {
            cost += crabs.iter().filter(|&&x| x == p).count() as i32 * triangular((i - p).abs());
        }
        if cost < least_cost {
            least_cost = cost;
        }
    }

    Box::new(least_cost)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 7, silver, gold)
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
        assert_eq!(silver("16,1,2,0,4,2,7,1,2,14").to_string(), "37");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 7);
        assert_eq!(silver(&input).to_string(), "355592");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("16,1,2,0,4,2,7,1,2,14").to_string(), "168");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 7);
        assert_eq!(gold(&input).to_string(), "101618069");
    }
}
