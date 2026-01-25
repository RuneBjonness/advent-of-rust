use crate::aoc_puzzle::{AocPuzzle, PuzzlePart};
use std::collections::HashMap;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let tiles = parse_tiles(input);
    let result = find_corner_product(&tiles);
    Box::new(result)
}

pub fn gold(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2020, 20, silver, gold).skip(PuzzlePart::Gold, "Not implemented in TypeScript")
}

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    img: Vec<String>,
    edges: Vec<String>,
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut current_id = 0;
    let mut current_img = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Tile") {
            // Save previous tile if exists
            if !current_img.is_empty() {
                let edges = get_edges(&current_img);
                tiles.push(Tile {
                    id: current_id,
                    img: current_img.clone(),
                    edges,
                });
                current_img.clear();
            }
            // Parse new tile ID
            current_id = line[5..9].parse().unwrap();
        } else {
            current_img.push(line.to_string());
        }
    }

    // Don't forget the last tile
    if !current_img.is_empty() {
        let edges = get_edges(&current_img);
        tiles.push(Tile {
            id: current_id,
            img: current_img,
            edges,
        });
    }

    tiles
}

fn get_edges(img: &[String]) -> Vec<String> {
    let size = img.len();
    vec![
        img[0].clone(),                                          // top
        img.iter().map(|r| r.chars().next().unwrap()).collect(), // left
        img[size - 1].clone(),                                   // bottom
        img.iter().map(|r| r.chars().last().unwrap()).collect(), // right
    ]
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn find_corner_product(tiles: &[Tile]) -> u64 {
    // Build a map of all edges (including reversed) to tile IDs
    let mut edge_map: HashMap<String, Vec<u64>> = HashMap::new();

    for tile in tiles {
        for edge in &tile.edges {
            edge_map.entry(edge.clone()).or_default().push(tile.id);
            edge_map
                .entry(reverse_string(edge))
                .or_default()
                .push(tile.id);
        }
    }

    // Find corner tiles (tiles with exactly 2 edges that match other tiles)
    let mut corner_ids = Vec::new();

    for tile in tiles {
        let mut matching_edges = 0;

        for edge in &tile.edges {
            // Check if this edge matches any other tile
            if let Some(tile_ids) = edge_map.get(edge) {
                // An edge matches if there's at least one other tile with this edge
                if tile_ids.iter().any(|&id| id != tile.id) {
                    matching_edges += 1;
                }
            }
        }

        // Corner tiles have exactly 2 matching edges
        if matching_edges == 2 {
            corner_ids.push(tile.id);
        }
    }

    corner_ids.iter().product()
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
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(silver(input).to_string(), "20899048083289");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2020, 20);
        assert_eq!(silver(&input).to_string(), "8425574315321");
    }

    #[test]
    fn gold_test_input() {
        let input = "test";
        assert_eq!(gold(input).to_string(), "0");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2020, 20);
        assert_eq!(gold(&input).to_string(), "0");
    }
}
