use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut lines = input.lines();
    let wire1_instructions = lines.next().unwrap();
    let wire2_instructions = lines.next().unwrap();

    let mut closest_intersection_dist = i32::MAX;

    let path1 = calculate_wire_path(wire1_instructions);
    let path2 = calculate_wire_path(wire2_instructions);

    for s1 in &path1 {
        for s2 in &path2 {
            let intersection = find_intersection(s1, s2).or_else(|| find_intersection(s2, s1));

            if let Some(intersection) = intersection {
                let manhattan_dist = intersection.x.abs() + intersection.y.abs();
                closest_intersection_dist = closest_intersection_dist.min(manhattan_dist);
            }
        }
    }

    Box::new(closest_intersection_dist)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut lines = input.lines();
    let wire1_instructions = lines.next().unwrap();
    let wire2_instructions = lines.next().unwrap();

    let mut closest_intersection_wire_length = i32::MAX;

    let path1 = calculate_wire_path(wire1_instructions);
    let path2 = calculate_wire_path(wire2_instructions);

    for s1 in &path1 {
        for s2 in &path2 {
            let intersection = find_intersection(s1, s2).or_else(|| find_intersection(s2, s1));

            if let Some(intersection) = intersection {
                closest_intersection_wire_length =
                    closest_intersection_wire_length.min(intersection.wire_length);
            }
        }
    }

    Box::new(closest_intersection_wire_length)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2019, 3, silver, gold)
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PathSection {
    from: Position,
    to: Position,
    direction: char,
    accumulated_length: i32,
}

#[derive(Debug)]
struct Intersection {
    x: i32,
    y: i32,
    wire_length: i32,
}

fn calculate_wire_path(wire_instructions: &str) -> Vec<PathSection> {
    let mut wire_path = Vec::new();
    let mut current_position = Position { x: 0, y: 0 };
    let mut accumulated_length = 0;

    for instruction in wire_instructions.split(',') {
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();
        let mut new_position = current_position;

        match direction {
            'R' => new_position.x += distance,
            'L' => new_position.x -= distance,
            'U' => new_position.y -= distance,
            'D' => new_position.y += distance,
            _ => unreachable!(),
        }

        wire_path.push(PathSection {
            from: current_position,
            to: new_position,
            direction,
            accumulated_length,
        });

        current_position = new_position;
        accumulated_length += distance;
    }

    wire_path
}

fn find_intersection(horizontal: &PathSection, vertical: &PathSection) -> Option<Intersection> {
    if (horizontal.direction == 'R' || horizontal.direction == 'L')
        && (vertical.direction == 'U' || vertical.direction == 'D')
    {
        if horizontal.from.x.min(horizontal.to.x) < vertical.from.x
            && vertical.from.x < horizontal.from.x.max(horizontal.to.x)
            && vertical.from.y.min(vertical.to.y) < horizontal.from.y
            && horizontal.from.y < vertical.from.y.max(vertical.to.y)
        {
            return Some(Intersection {
                x: vertical.from.x,
                y: horizontal.from.y,
                wire_length: horizontal.accumulated_length
                    + vertical.accumulated_length
                    + (vertical.from.x - horizontal.from.x).abs()
                    + (horizontal.from.y - vertical.from.y).abs(),
            });
        }
    }
    None
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
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(silver(input).to_string(), "159");
    }

    #[test]
    fn silver_test_input_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(silver(input).to_string(), "135");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2019, 3);
        assert_eq!(silver(&input).to_string(), "721");
    }

    #[test]
    fn gold_test_input() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(gold(input).to_string(), "610");
    }

    #[test]
    fn gold_test_input_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(gold(input).to_string(), "410");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2019, 3);
        assert_eq!(gold(&input).to_string(), "7388");
    }
}
