use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut m = vec![vec![0u16; 1000]; 1000];

    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();

        let p1: Vec<usize> = points[0]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let p2: Vec<usize> = points[1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let (p1x, p1y) = (p1[0], p1[1]);
        let (p2x, p2y) = (p2[0], p2[1]);

        if p1x == p2x {
            let min_y = p1y.min(p2y);
            let max_y = p1y.max(p2y);
            for y in min_y..=max_y {
                m[p1x][y] += 1;
            }
        } else if p1y == p2y {
            let min_x = p1x.min(p2x);
            let max_x = p1x.max(p2x);
            for x in min_x..=max_x {
                m[x][p1y] += 1;
            }
        }
    }

    let count = m.iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x > 1)
        .count();

    Box::new(count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut m = vec![vec![0u16; 1000]; 1000];

    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();

        let p1: Vec<i32> = points[0]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let p2: Vec<i32> = points[1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let (p1x, p1y) = (p1[0], p1[1]);
        let (p2x, p2y) = (p2[0], p2[1]);

        if p1x == p2x {
            let min_y = p1y.min(p2y);
            let max_y = p1y.max(p2y);
            for y in min_y..=max_y {
                m[p1x as usize][y as usize] += 1;
            }
        } else if p1y == p2y {
            let min_x = p1x.min(p2x);
            let max_x = p1x.max(p2x);
            for x in min_x..=max_x {
                m[x as usize][p1y as usize] += 1;
            }
        } else if (p2x - p1x).abs() == (p2y - p1y).abs() {
            let mut x = p1x;
            let mut y = p1y;
            for _ in 0..=(p2x - p1x).abs() {
                m[x as usize][y as usize] += 1;
                x = if p1x < p2x { x + 1 } else { x - 1 };
                y = if p1y < p2y { y + 1 } else { y - 1 };
            }
        }
    }

    let count = m.iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x > 1)
        .count();

    Box::new(count)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 5, silver, gold)
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
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(silver(input).to_string(), "5");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 5);
        assert_eq!(silver(&input).to_string(), "5197");
    }

    #[test]
    fn gold_test_input() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(gold(input).to_string(), "12");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 5);
        assert_eq!(gold(&input).to_string(), "18605");
    }
}
