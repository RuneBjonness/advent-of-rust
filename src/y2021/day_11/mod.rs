use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut octopuses: Vec<Vec<u32>> = input
        .lines()
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut flash_count = 0;

    for _ in 0..100 {
        octopuses = octopuses
            .iter()
            .map(|r| r.iter().map(|&c| c + 1).collect())
            .collect();
        flash_count += step(&mut octopuses);
        octopuses = octopuses
            .iter()
            .map(|r| r.iter().map(|&c| if c > 9 { 0 } else { c }).collect())
            .collect();
    }

    Box::new(flash_count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut octopuses: Vec<Vec<u32>> = input
        .lines()
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut step_count = 0;

    loop {
        octopuses = octopuses
            .iter()
            .map(|r| r.iter().map(|&c| c + 1).collect())
            .collect();
        let flash_count = step(&mut octopuses);
        step_count += 1;
        if flash_count == 100 {
            return Box::new(step_count);
        }
        octopuses = octopuses
            .iter()
            .map(|r| r.iter().map(|&c| if c > 9 { 0 } else { c }).collect())
            .collect();
    }
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 11, silver, gold)
}

fn step(m: &mut Vec<Vec<u32>>) -> usize {
    let new_flashes = get_flashing_points(m);
    let mut flash_count = new_flashes.len();
    if flash_count == 0 {
        return flash_count;
    }

    for &(r, c) in &new_flashes {
        m[r][c] = 0;
    }

    for &(r, c) in &new_flashes {
        increment_neighbours(m, r, c);
        flash_count += step(m);
    }

    flash_count
}

fn get_flashing_points(m: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for r in 0..m.len() {
        for c in 0..m[r].len() {
            if m[r][c] == 10 {
                points.push((r, c));
            }
        }
    }
    points
}

fn increment_neighbours(m: &mut Vec<Vec<u32>>, r: usize, c: usize) {
    let rows = m.len();
    let cols = m[r].len();

    if c > 0 {
        increment(m, r, c - 1);
    }
    if c + 1 < cols {
        increment(m, r, c + 1);
    }
    if r > 0 {
        increment(m, r - 1, c);
        if c > 0 {
            increment(m, r - 1, c - 1);
        }
        if c + 1 < cols {
            increment(m, r - 1, c + 1);
        }
    }
    if r + 1 < rows {
        increment(m, r + 1, c);
        if c > 0 {
            increment(m, r + 1, c - 1);
        }
        if c + 1 < cols {
            increment(m, r + 1, c + 1);
        }
    }
}

fn increment(m: &mut Vec<Vec<u32>>, r: usize, c: usize) {
    if m[r][c] > 0 {
        m[r][c] += 1;
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

    #[test]
    fn silver_test_input() {
        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        assert_eq!(silver(input).to_string(), "1656");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 11);
        assert_eq!(silver(&input).to_string(), "1661");
    }

    #[test]
    fn gold_test_input() {
        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        assert_eq!(gold(input).to_string(), "195");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 11);
        assert_eq!(gold(&input).to_string(), "334");
    }
}
