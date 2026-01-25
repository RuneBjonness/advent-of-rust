use crate::aoc_puzzle::{AocPuzzle, PuzzlePart};
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Tile {
    row: usize,
    col: usize,
    val: u32,
    sum: u32,
    resolved_top: u32,
    resolved_right: u32,
    resolved_bottom: u32,
    resolved_left: u32,
}

impl Tile {
    fn new(row: usize, col: usize, val: u32, max_row: usize, max_col: usize) -> Self {
        Self {
            row,
            col,
            val,
            sum: 0,
            resolved_top: if row == 0 { u32::MAX } else { 0 },
            resolved_right: if col == max_col { u32::MAX } else { 0 },
            resolved_bottom: if row == max_row { u32::MAX } else { 0 },
            resolved_left: if col == 0 { u32::MAX } else { 0 },
        }
    }

    fn has_unresolved_neighbours(&self) -> bool {
        self.resolved_top == 0
            || self.resolved_right == 0
            || self.resolved_bottom == 0
            || self.resolved_left == 0
    }
}

fn get_neighbours(m: &mut Vec<Vec<Tile>>, r: usize, c: usize) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let max_row = m.len() - 1;
    let max_col = m[0].len() - 1;

    if r > 0 && m[r][c].resolved_top == 0 {
        tiles.push(m[r - 1][c].clone());
    }
    if r < max_row && m[r][c].resolved_bottom == 0 {
        tiles.push(m[r + 1][c].clone());
    }
    if c > 0 && m[r][c].resolved_left == 0 {
        tiles.push(m[r][c - 1].clone());
    }
    if c < max_col && m[r][c].resolved_right == 0 {
        tiles.push(m[r][c + 1].clone());
    }

    tiles
}

fn check_tile(t: &mut Tile) {
    if t.sum == 0 && !t.has_unresolved_neighbours() {
        t.sum = t
            .resolved_top
            .min(t.resolved_right)
            .min(t.resolved_bottom)
            .min(t.resolved_left)
            + t.val;
    }
}

fn set_tile_sum(m: &mut Vec<Vec<Tile>>, r: usize, c: usize, sum: u32) {
    let max_row = m.len() - 1;
    let max_col = m[0].len() - 1;

    m[r][c].sum = sum;

    if r > 0 {
        m[r - 1][c].resolved_bottom = sum;
        let tile = &mut m[r - 1][c];
        check_tile(tile);
    }
    if r < max_row {
        m[r + 1][c].resolved_top = sum;
        let tile = &mut m[r + 1][c];
        check_tile(tile);
    }
    if c > 0 {
        m[r][c - 1].resolved_right = sum;
        let tile = &mut m[r][c - 1];
        check_tile(tile);
    }
    if c < max_col {
        m[r][c + 1].resolved_left = sum;
        let tile = &mut m[r][c + 1];
        check_tile(tile);
    }
}

fn next_val(val: u32, increment: u32) -> u32 {
    let n = (val + increment) % 9;
    if n > 0 {
        n
    } else {
        9
    }
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let values: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let max_row = values.len() - 1;
    let max_col = values[0].len() - 1;

    let mut m: Vec<Vec<Tile>> = values
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &val)| Tile::new(i, j, val, max_row, max_col))
                .collect()
        })
        .collect();

    let initial_val = m[0][0].val;
    set_tile_sum(&mut m, 0, 0, initial_val);

    while m[max_row][max_col].sum == 0 {
        let mut cheapest: Option<&Tile> = None;
        for row in &m {
            for tile in row {
                if tile.sum > 0 && tile.has_unresolved_neighbours() {
                    if let Some(current_cheapest) = cheapest {
                        if tile.sum < current_cheapest.sum {
                            cheapest = Some(tile);
                        }
                    } else {
                        cheapest = Some(tile);
                    }
                }
            }
        }

        if let Some(cheapest_tile) = cheapest {
            let r = cheapest_tile.row;
            let c = cheapest_tile.col;
            let sum = cheapest_tile.sum;
            let neighbours = get_neighbours(&mut m, r, c);

            for t in neighbours {
                set_tile_sum(&mut m, t.row, t.col, t.val + sum);
            }
        }
    }

    Box::new(m[max_row][max_col].sum - m[0][0].sum)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let template: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let template_height = template.len();
    let template_width = template[0].len();
    let full_height = template_height * 5;
    let full_width = template_width * 5;

    // Build the full map
    let mut values = vec![vec![0u32; full_width]; full_height];
    for i in 0..5 {
        for j in 0..5 {
            for (ri, row) in template.iter().enumerate() {
                for (ci, &val) in row.iter().enumerate() {
                    values[template_height * i + ri][template_width * j + ci] =
                        next_val(val, (i + j) as u32);
                }
            }
        }
    }

    let max_row = full_height - 1;
    let max_col = full_width - 1;

    let mut m: Vec<Vec<Tile>> = values
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &val)| Tile::new(i, j, val, max_row, max_col))
                .collect()
        })
        .collect();

    let initial_val = m[0][0].val;
    set_tile_sum(&mut m, 0, 0, initial_val);

    while m[max_row][max_col].sum == 0 {
        let mut cheapest: Option<&Tile> = None;
        for row in &m {
            for tile in row {
                if tile.sum > 0 && tile.has_unresolved_neighbours() {
                    if let Some(current_cheapest) = cheapest {
                        if tile.sum < current_cheapest.sum {
                            cheapest = Some(tile);
                        }
                    } else {
                        cheapest = Some(tile);
                    }
                }
            }
        }

        if let Some(cheapest_tile) = cheapest {
            let r = cheapest_tile.row;
            let c = cheapest_tile.col;
            let sum = cheapest_tile.sum;
            let neighbours = get_neighbours(&mut m, r, c);

            for t in neighbours {
                set_tile_sum(&mut m, t.row, t.col, t.val + sum);
            }
        }
    }

    let start_val = m[0][0].sum;
    Box::new(m[max_row][max_col].sum - start_val)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 15, silver, gold)
        .skip(PuzzlePart::Gold, "Very slow - takes ~3 minutes")
        .skip(PuzzlePart::Both, "Very slow - takes ~3 minutes")
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

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "40");
    }

    #[test]
    #[ignore] // Slow - takes ~0.18s in release mode
    fn silver_actual_input() {
        let input = read_input(2021, 15);
        assert_eq!(silver(&input).to_string(), "609");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "315");
    }

    #[test]
    #[ignore] // Very slow - takes ~3+ minutes in release mode
    fn gold_actual_input() {
        let input = read_input(2021, 15);
        assert_eq!(gold(&input).to_string(), "2925");
    }
}
