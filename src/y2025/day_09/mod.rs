use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let positions: Vec<Vec2> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            Vec2::new(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut max_area: i64 = 0;
    for a in 0..positions.len() - 1 {
        for b in a + 1..positions.len() {
            let area = ((positions[b].x - positions[a].x).abs() as i64 + 1)
                * ((positions[b].y - positions[a].y).abs() as i64 + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Box::new(max_area)
}

#[derive(Debug, Clone)]
struct Edge {
    min: i32,
    max: i32,
    index: i32,
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let positions: Vec<Vec2> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            Vec2::new(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut horizontal_edges: Vec<Edge> = Vec::new();
    let mut vertical_edges: Vec<Edge> = Vec::new();

    let mut from = positions[0];
    let mut positions_with_loop = positions.clone();
    positions_with_loop.push(from);

    for i in 1..positions_with_loop.len() {
        let to = positions_with_loop[i];
        if from.x == to.x {
            vertical_edges.push(Edge {
                min: from.y.min(to.y),
                max: from.y.max(to.y),
                index: from.x,
            });
        } else if from.y == to.y {
            horizontal_edges.push(Edge {
                min: from.x.min(to.x),
                max: from.x.max(to.x),
                index: from.y,
            });
        }
        from = to;
    }

    vertical_edges.sort_by_key(|e| e.index);
    horizontal_edges.sort_by_key(|e| e.index);

    let mut max_area: i64 = 0;
    for a in 0..positions.len() - 1 {
        for b in a + 1..positions.len() {
            let min_x = positions[a].x.min(positions[b].x);
            let max_x = positions[a].x.max(positions[b].x);
            let min_y = positions[a].y.min(positions[b].y);
            let max_y = positions[a].y.max(positions[b].y);

            let area = (max_x - min_x + 1) as i64 * (max_y - min_y + 1) as i64;

            if area > max_area
                && is_valid_area(
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    &horizontal_edges,
                    &vertical_edges,
                )
            {
                max_area = area;
            }
        }
    }
    Box::new(max_area)
}

fn intersects(edge: &Edge, p: i32) -> bool {
    p > edge.min && p < edge.max
}

fn is_valid_area(
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    horizontal_edges: &[Edge],
    vertical_edges: &[Edge],
) -> bool {
    let mut i = horizontal_edges
        .iter()
        .position(|e| e.index > min_y)
        .unwrap_or(horizontal_edges.len());
    while i < horizontal_edges.len() && horizontal_edges[i].index < max_y {
        if intersects(&horizontal_edges[i], min_x + 1)
            || intersects(&horizontal_edges[i], max_x - 1)
        {
            return false;
        }
        i += 1;
    }

    let mut i = vertical_edges
        .iter()
        .position(|e| e.index > min_x)
        .unwrap_or(vertical_edges.len());
    while i < vertical_edges.len() && vertical_edges[i].index < max_x {
        if intersects(&vertical_edges[i], min_y + 1) || intersects(&vertical_edges[i], max_y - 1) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 9, silver, gold)
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

    const TEST_INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "50");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 9);
        assert_eq!(silver(&input).to_string(), "4740155680");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "24");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 9);
        assert_eq!(gold(&input).to_string(), "1543501936");
    }
}
