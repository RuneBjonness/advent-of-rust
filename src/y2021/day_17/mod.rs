use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

// These magic numbers were found via trial and error to limit the search space
// Will not work for all inputs!
const MAGIC_NUMBER_MIN_X: i32 = 0;
const MAGIC_NUMBER_MAX_X: i32 = 50;
const MAGIC_NUMBER_MIN_Y: i32 = 0;
const MAGIC_NUMBER_MAX_Y: i32 = 400;

pub fn silver(input: &str) -> Box<dyn Display> {
    let target_area = parse_target_area(input);
    let mut max_y_pos = i32::MIN;
    let mut y = MAGIC_NUMBER_MIN_Y;

    while y < MAGIC_NUMBER_MAX_Y {
        for x in MAGIC_NUMBER_MIN_X..MAGIC_NUMBER_MAX_X {
            let top = launch(x, y, &target_area);
            if top > max_y_pos {
                max_y_pos = top;
                break;
            }
        }
        y += 1;
    }

    Box::new(max_y_pos)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let target_area = parse_target_area(input);
    let mut hits = 0;

    for x in MAGIC_NUMBER_MIN_X..=target_area.max_x {
        for y in target_area.min_y..=MAGIC_NUMBER_MAX_Y {
            if hit(x, y, &target_area) {
                hits += 1;
            }
        }
    }

    Box::new(hits)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 17, silver, gold)
}

fn hit(x: i32, y: i32, target_area: &TargetArea) -> bool {
    launch(x, y, target_area) > i32::MIN
}

fn launch(mut x: i32, mut y: i32, target_area: &TargetArea) -> i32 {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut max_y = 0;

    loop {
        pos_x += x;
        pos_y += y;
        if pos_y > max_y {
            max_y = pos_y;
        }

        if pos_x >= target_area.min_x
            && pos_x <= target_area.max_x
            && pos_y >= target_area.min_y
            && pos_y <= target_area.max_y
        {
            return max_y;
        }

        if pos_x > target_area.max_x || pos_y < target_area.min_y {
            return i32::MIN;
        }

        if x < 0 {
            x += 1;
        } else if x > 0 {
            x -= 1;
        }
        y -= 1;
    }
}

struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn parse_target_area(input: &str) -> TargetArea {
    let re = regex::Regex::new(r"x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    TargetArea {
        min_x: caps[1].parse().unwrap(),
        max_x: caps[2].parse().unwrap(),
        min_y: caps[3].parse().unwrap(),
        max_y: caps[4].parse().unwrap(),
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
        assert_eq!(silver("target area: x=20..30, y=-10..-5").to_string(), "45");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 17);
        assert_eq!(silver(&input).to_string(), "6555");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("target area: x=20..30, y=-10..-5").to_string(), "112");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 17);
        assert_eq!(gold(&input).to_string(), "4973");
    }
}
