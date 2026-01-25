use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let heightmap = parse_heightmap(input);
    let mut total_risk_level = 0;

    for r in 0..heightmap.len() {
        for c in 0..heightmap[r].len() {
            if get_neighbours(&heightmap, r, c)
                .iter()
                .all(|&x| x > heightmap[r][c])
            {
                total_risk_level += heightmap[r][c] + 1;
            }
        }
    }

    Box::new(total_risk_level)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let heightmap = parse_heightmap(input);
    let mut lowpoints = Vec::new();

    for r in 0..heightmap.len() {
        for c in 0..heightmap[r].len() {
            if get_neighbours(&heightmap, r, c)
                .iter()
                .all(|&x| x > heightmap[r][c])
            {
                lowpoints.push((r, c));
            }
        }
    }

    let mut basins: Vec<i32> = lowpoints
        .iter()
        .map(|&p| basin(p, &mut heightmap.clone()))
        .collect();

    basins.sort_by(|a, b| b.cmp(a));

    Box::new(basins[0] * basins[1] * basins[2])
}

fn parse_heightmap(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn basin(lp: (usize, usize), m: &mut Vec<Vec<i32>>) -> i32 {
    let (r, c) = lp;
    let tiles = get_higher_neighbours(m, r, c);

    if m[r][c] > 9 {
        return 0;
    }
    m[r][c] += 10;

    if tiles.is_empty() {
        return 1;
    }

    let mut sum = 1;
    for t in tiles {
        sum += basin(t, m);
    }
    sum
}

fn get_higher_neighbours(m: &Vec<Vec<i32>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    get_neighbour_points(m, r, c)
        .into_iter()
        .filter(|(_nr, _nc, val)| *val < 9 && *val > m[r][c])
        .map(|(nr, nc, _)| (nr, nc))
        .collect()
}

fn get_neighbour_points(m: &Vec<Vec<i32>>, r: usize, c: usize) -> Vec<(usize, usize, i32)> {
    let mut tiles = Vec::new();

    if r > 0 {
        tiles.push((r - 1, c, m[r - 1][c]));
    }
    if r < m.len() - 1 {
        tiles.push((r + 1, c, m[r + 1][c]));
    }
    if c > 0 {
        tiles.push((r, c - 1, m[r][c - 1]));
    }
    if c < m[0].len() - 1 {
        tiles.push((r, c + 1, m[r][c + 1]));
    }

    tiles
}

fn get_neighbours(m: &Vec<Vec<i32>>, r: usize, c: usize) -> Vec<i32> {
    let mut tiles = Vec::new();

    if r > 0 {
        tiles.push(m[r - 1][c]);
    }
    if r < m.len() - 1 {
        tiles.push(m[r + 1][c]);
    }
    if c > 0 {
        tiles.push(m[r][c - 1]);
    }
    if c < m[0].len() - 1 {
        tiles.push(m[r][c + 1]);
    }

    tiles
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 9, silver, gold)
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
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(silver(input).to_string(), "15");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 9);
        assert_eq!(silver(&input).to_string(), "500");
    }

    #[test]
    fn gold_test_input() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(gold(input).to_string(), "1134");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 9);
        assert_eq!(gold(&input).to_string(), "970200");
    }
}
