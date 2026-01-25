use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashSet;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut edges = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let a = parts[0];
        let b = parts[1];
        edges.push((a.to_string(), b.to_string()));
        edges.push((b.to_string(), a.to_string()));
    }

    // Filter out edges that go to "start" or from "end"
    edges.retain(|e| e.1 != "start" && e.0 != "end");

    let mut paths = Vec::new();
    get_paths_from(
        "start",
        &edges,
        Vec::new(),
        &mut paths,
        get_valid_destinations_silver,
    );

    Box::new(paths.len())
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut edges = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let a = parts[0];
        let b = parts[1];
        edges.push((a.to_string(), b.to_string()));
        edges.push((b.to_string(), a.to_string()));
    }

    // Filter out edges that go to "start" or from "end"
    edges.retain(|e| e.1 != "start" && e.0 != "end");

    let mut paths = Vec::new();
    get_paths_from(
        "start",
        &edges,
        Vec::new(),
        &mut paths,
        get_valid_destinations_gold,
    );

    Box::new(paths.len())
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 12, silver, gold)
}

fn is_big_cave(node: &str) -> bool {
    node.chars().all(|c| c.is_uppercase())
}

fn has_visited_any_small_cave_twice(path: &[String]) -> bool {
    let small_caves: Vec<&String> = path.iter().filter(|p| !is_big_cave(p)).collect();
    let unique: HashSet<&String> = small_caves.iter().copied().collect();
    small_caves.len() != unique.len()
}

fn get_valid_destinations_silver(
    node: &str,
    edges: &[(String, String)],
    path: &[String],
) -> Vec<String> {
    edges
        .iter()
        .filter(|(from, to)| from == node && (is_big_cave(to) || !path.contains(to)))
        .map(|(_, to)| to.clone())
        .collect()
}

fn get_valid_destinations_gold(
    node: &str,
    edges: &[(String, String)],
    path: &[String],
) -> Vec<String> {
    edges
        .iter()
        .filter(|(from, to)| {
            from == node
                && (is_big_cave(to)
                    || !path.contains(to)
                    || !has_visited_any_small_cave_twice(path))
        })
        .map(|(_, to)| to.clone())
        .collect()
}

fn get_paths_from(
    node: &str,
    edges: &[(String, String)],
    mut path: Vec<String>,
    paths: &mut Vec<Vec<String>>,
    get_valid_destinations: fn(&str, &[(String, String)], &[String]) -> Vec<String>,
) {
    path.push(node.to_string());
    if node == "end" {
        paths.push(path);
        return;
    }
    let destinations = get_valid_destinations(node, edges, &path);
    for dest in destinations {
        get_paths_from(&dest, edges, path.clone(), paths, get_valid_destinations);
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
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        assert_eq!(silver(input).to_string(), "10");
    }

    #[test]
    fn silver_test_input_larger() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        assert_eq!(silver(input).to_string(), "19");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 12);
        assert_eq!(silver(&input).to_string(), "3292");
    }

    #[test]
    fn gold_test_input() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        assert_eq!(gold(input).to_string(), "36");
    }

    #[test]
    fn gold_test_input_larger() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        assert_eq!(gold(input).to_string(), "103");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 12);
        assert_eq!(gold(&input).to_string(), "89592");
    }
}
