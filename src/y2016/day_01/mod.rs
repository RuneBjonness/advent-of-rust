use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn vector(self, length: i32) -> (i32, i32) {
        match self {
            Direction::Up => (0, -length),
            Direction::Right => (length, 0),
            Direction::Down => (0, length),
            Direction::Left => (-length, 0),
        }
    }

    fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut x = 0;
    let mut y = 0;
    let mut dir = Direction::Up;

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let length = instruction[1..].parse::<i32>().unwrap();

        dir = if turn == 'R' {
            dir.turn_right()
        } else {
            dir.turn_left()
        };

        let (dx, dy) = dir.vector(length);
        x += dx;
        y += dy;
    }

    Box::new(x.abs() + y.abs())
}

#[derive(Clone, Copy)]
struct Path {
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
}

impl Path {
    fn intersects_h_v(h_path: &Path, v_path: &Path) -> Option<(i32, i32)> {
        let h_min_x = h_path.from_x.min(h_path.to_x);
        let h_max_x = h_path.from_x.max(h_path.to_x);
        let v_min_y = v_path.from_y.min(v_path.to_y);
        let v_max_y = v_path.from_y.max(v_path.to_y);

        if v_path.from_x >= h_min_x
            && v_path.from_x <= h_max_x
            && h_path.from_y >= v_min_y
            && h_path.from_y <= v_max_y
        {
            return Some((v_path.from_x, h_path.from_y));
        }
        None
    }
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut x = 0;
    let mut y = 0;
    let mut horizontal_paths: Vec<Path> = Vec::new();
    let mut vertical_paths: Vec<Path> = Vec::new();
    let mut dir = Direction::Up;

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let length = instruction[1..].parse::<i32>().unwrap();

        dir = if turn == 'R' {
            dir.turn_right()
        } else {
            dir.turn_left()
        };

        let (from_dx, from_dy) = dir.vector(1);
        let (to_dx, to_dy) = dir.vector(length);

        let path = Path {
            from_x: x + from_dx,
            from_y: y + from_dy,
            to_x: x + to_dx,
            to_y: y + to_dy,
        };

        if dir.is_horizontal() {
            for v_path in &vertical_paths {
                if let Some((ix, iy)) = Path::intersects_h_v(&path, v_path) {
                    return Box::new(ix.abs() + iy.abs());
                }
            }
            horizontal_paths.push(path);
        } else {
            for h_path in &horizontal_paths {
                if let Some((ix, iy)) = Path::intersects_h_v(h_path, &path) {
                    return Box::new(ix.abs() + iy.abs());
                }
            }
            vertical_paths.push(path);
        }

        x += to_dx;
        y += to_dy;
    }

    Box::new(0)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2016, 1, silver, gold)
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
        assert_eq!(silver("R5, L5, R5, R3").to_string(), "12");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2016, 1);
        assert_eq!(silver(&input).to_string(), "209");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("R8, R4, R4, R8").to_string(), "4");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2016, 1);
        assert_eq!(gold(&input).to_string(), "136");
    }
}
