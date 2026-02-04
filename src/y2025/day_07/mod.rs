use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut split_count = 0;

    for row in 1..grid.len() {
        for col in 0..grid[row].len() {
            let prev = grid[row - 1][col];
            if prev == '|' || prev == 'S' {
                if grid[row][col] == '.' {
                    grid[row][col] = '|';
                } else if grid[row][col] == '^' {
                    grid[row][col - 1] = '|';
                    grid[row][col + 1] = '|';
                    split_count += 1;
                }
            }
        }
    }

    Box::new(split_count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut grid: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    'S' => 1,
                    _ => -1,
                })
                .collect()
        })
        .collect();

    for row in 1..grid.len() {
        for col in 0..grid[row].len() {
            let prev = grid[row - 1][col];
            if prev > 0 {
                if grid[row][col] >= 0 {
                    grid[row][col] += prev;
                } else {
                    grid[row][col - 1] += prev;
                    grid[row][col + 1] += prev;
                }
            }
        }
    }

    let result: i64 = grid.last().unwrap().iter().sum();
    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 7, silver, gold)
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

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "21");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 7);
        assert_eq!(silver(&input).to_string(), "1581");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "40");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 7);
        assert_eq!(gold(&input).to_string(), "73007003089792");
    }
}
