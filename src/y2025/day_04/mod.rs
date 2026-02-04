use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

const ADJACENT_DELTAS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub fn silver(input: &str) -> Box<dyn Display> {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let grid: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();

    let mut count = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row * num_cols + col] == '@' {
                let mut adjacent_count = 0;
                for &(dr, dc) in &ADJACENT_DELTAS {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    if new_row >= 0 && new_row < num_rows as i32
                        && new_col >= 0 && new_col < num_cols as i32 {
                        let neighbor_idx = new_row as usize * num_cols + new_col as usize;
                        if grid[neighbor_idx] == '@' {
                            adjacent_count += 1;
                            if adjacent_count >= 4 {
                                break;
                            }
                        }
                    }
                }
                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    Box::new(count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let result = both(input);
    Box::new(result.1)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    // Create grid where '@' is 0, everything else is -1
    let mut grid: Vec<i32> = lines
        .iter()
        .flat_map(|line| line.chars().map(|c| if c == '@' { 0 } else { -1 }))
        .collect();

    let mut to_remove: Vec<usize> = Vec::new();

    // Count adjacent @ cells for all cells
    for row in 0..num_rows {
        for col in 0..num_cols {
            let idx = row * num_cols + col;
            if grid[idx] == 0 {
                let mut adjacent_count = 0;
                for &(dr, dc) in &ADJACENT_DELTAS {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    if new_row >= 0 && new_row < num_rows as i32
                        && new_col >= 0 && new_col < num_cols as i32 {
                        let neighbor_idx = new_row as usize * num_cols + new_col as usize;
                        if grid[neighbor_idx] >= 0 {
                            adjacent_count += 1;
                        }
                    }
                }
                if adjacent_count < 4 {
                    to_remove.push(idx);
                }
                grid[idx] = adjacent_count;
            }
        }
    }

    let silver_count = to_remove.len();
    let mut gold_count = 0;

    while let Some(idx) = to_remove.pop() {
        if grid[idx] < 0 {
            continue;
        }
        gold_count += 1;
        grid[idx] = -1;

        let row = idx / num_cols;
        let col = idx % num_cols;

        for &(dr, dc) in &ADJACENT_DELTAS {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if new_row >= 0 && new_row < num_rows as i32
                && new_col >= 0 && new_col < num_cols as i32 {
                let neighbor_idx = new_row as usize * num_cols + new_col as usize;
                grid[neighbor_idx] -= 1;
                if grid[neighbor_idx] >= 0 && grid[neighbor_idx] < 4 {
                    to_remove.push(neighbor_idx);
                }
            }
        }
    }

    (Box::new(silver_count), Box::new(gold_count))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 4, silver, gold).with_both(both)
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

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "13");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 4);
        assert_eq!(silver(&input).to_string(), "1372");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "43");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 4);
        assert_eq!(gold(&input).to_string(), "7922");
    }
}
