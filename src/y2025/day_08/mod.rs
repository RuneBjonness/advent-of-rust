use crate::aoc_puzzle::AocPuzzle;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 { x, y, z }
    }

    fn distance(&self, other: &Vec3) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Clone)]
struct DistanceCache {
    a: Vec3,
    b: Vec3,
    distance: f64,
}

// Wrapper for min-heap behavior
struct MinHeapEntry(DistanceCache);

impl PartialEq for MinHeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance == other.0.distance
    }
}

impl Eq for MinHeapEntry {}

impl PartialOrd for MinHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior
        other
            .0
            .distance
            .partial_cmp(&self.0.distance)
            .unwrap_or(Ordering::Equal)
    }
}

// Wrapper for max-heap behavior
struct MaxHeapEntry(DistanceCache);

impl PartialEq for MaxHeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance == other.0.distance
    }
}

impl Eq for MaxHeapEntry {}

impl PartialOrd for MaxHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaxHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .distance
            .partial_cmp(&other.0.distance)
            .unwrap_or(Ordering::Equal)
    }
}

fn parse_positions(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Vec3::new(parts[0], parts[1], parts[2])
        })
        .collect()
}

fn calculate_distances(positions: &[Vec3]) -> BinaryHeap<MinHeapEntry> {
    let mut heap = BinaryHeap::new();
    for i in 0..positions.len() - 1 {
        let pos_a = positions[i];
        for j in i + 1..positions.len() {
            let pos_b = positions[j];
            let dist = pos_a.distance(&pos_b);
            heap.push(MinHeapEntry(DistanceCache {
                a: pos_a,
                b: pos_b,
                distance: dist,
            }));
        }
    }
    heap
}

fn calculate_closest_distances(positions: &[Vec3], n: usize) -> BinaryHeap<MinHeapEntry> {
    let mut max_heap: BinaryHeap<MaxHeapEntry> = BinaryHeap::new();
    for i in 0..positions.len() - 1 {
        let pos_a = positions[i];
        for j in i + 1..positions.len() {
            let pos_b = positions[j];
            let dist = pos_a.distance(&pos_b);
            if max_heap.len() >= n {
                if let Some(max_entry) = max_heap.peek() {
                    if dist < max_entry.0.distance {
                        max_heap.pop();
                        max_heap.push(MaxHeapEntry(DistanceCache {
                            a: pos_a,
                            b: pos_b,
                            distance: dist,
                        }));
                    }
                }
            } else {
                max_heap.push(MaxHeapEntry(DistanceCache {
                    a: pos_a,
                    b: pos_b,
                    distance: dist,
                }));
            }
        }
    }

    // Convert max heap to min heap
    let mut min_heap = BinaryHeap::new();
    while let Some(entry) = max_heap.pop() {
        min_heap.push(MinHeapEntry(entry.0));
    }
    min_heap
}

fn solve_both(
    input: &str,
    skip_silver: bool,
    skip_gold: bool,
    silver_connections: usize,
) -> (i32, i32) {
    let positions = parse_positions(input);
    let mut distances = if skip_gold {
        calculate_closest_distances(&positions, silver_connections)
    } else {
        calculate_distances(&positions)
    };

    let mut circuits: Vec<Vec<Vec3>> = Vec::new();
    let mut count = 0;
    let mut silver_result = 0;
    let mut gold_result = 0;

    while let Some(entry) = distances.pop() {
        count += 1;
        let DistanceCache { a, b, .. } = entry.0;

        // Check if both a and b are already in the same circuit
        if !circuits
            .iter()
            .any(|circuit| circuit.contains(&a) && circuit.contains(&b))
        {
            let circuit_with_a_idx = circuits.iter().position(|c| c.contains(&a));
            let circuit_with_b_idx = circuits.iter().position(|c| c.contains(&b));

            match (circuit_with_a_idx, circuit_with_b_idx) {
                (Some(a_idx), Some(b_idx)) if a_idx != b_idx => {
                    // Merge circuits
                    let circuit_b = circuits.remove(b_idx.max(a_idx));
                    let circuit_a_idx = if b_idx > a_idx { a_idx } else { b_idx };
                    circuits[circuit_a_idx].extend(circuit_b);
                }
                (Some(a_idx), None) => {
                    circuits[a_idx].push(b);
                }
                (None, Some(b_idx)) => {
                    circuits[b_idx].push(a);
                }
                (None, None) => {
                    circuits.push(vec![a, b]);
                }
                _ => {}
            }
        }

        if !skip_silver && silver_result == 0 && count >= silver_connections {
            let mut circuit_sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
            circuit_sizes.sort_by(|a, b| b.cmp(a));
            silver_result = (circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]) as i32;
        }

        if circuits.len() == 1 && circuits[0].len() == positions.len() {
            gold_result = a.x * b.x;
            break;
        }
    }

    (silver_result, gold_result)
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let (result, _) = solve_both(input, false, true, 1000);
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let (_, result) = solve_both(input, true, false, 1000);
    Box::new(result)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let (silver, gold) = solve_both(input, false, false, 1000);
    (Box::new(silver), Box::new(gold))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 8, silver, gold).with_both(both)
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

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn silver_test_input() {
        // Test with 10 connections instead of 1000 for smaller test input
        let (result, _) = solve_both(TEST_INPUT, false, true, 10);
        assert_eq!(result.to_string(), "40");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 8);
        assert_eq!(silver(&input).to_string(), "63920");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "25272");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 8);
        assert_eq!(gold(&input).to_string(), "1026594680");
    }
}
