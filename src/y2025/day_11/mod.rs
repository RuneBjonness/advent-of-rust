use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashMap;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let devices = parse_devices(input);
    Box::new(count_paths(&devices, "you", "out"))
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let devices = parse_devices(input);
    let mut cache = HashMap::new();
    let path_counts = count_paths_with_nodes(&devices, "svr", "out", &["fft", "dac"], &mut cache);
    let result = path_counts
        .iter()
        .find(|pc| pc.required_devices == 2)
        .map(|pc| pc.path_count)
        .unwrap_or(0);
    Box::new(result)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let devices = parse_devices(input);
    let silver_result = count_paths(&devices, "you", "out");

    let mut cache = HashMap::new();
    let path_counts = count_paths_with_nodes(&devices, "svr", "out", &["fft", "dac"], &mut cache);
    let gold_result = path_counts
        .iter()
        .find(|pc| pc.required_devices == 2)
        .map(|pc| pc.path_count)
        .unwrap_or(0);

    (Box::new(silver_result), Box::new(gold_result))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 11, silver, gold).with_both(both)
}

fn parse_devices(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let device = parts[0];
            let connections = parts[1].split(' ').collect();
            (device, connections)
        })
        .collect()
}

fn count_paths(devices: &HashMap<&str, Vec<&str>>, current: &str, target: &str) -> i64 {
    if current == target {
        return 1;
    }
    let mut path_count = 0;
    if let Some(outputs) = devices.get(current) {
        for output in outputs {
            path_count += count_paths(devices, output, target);
        }
    }
    path_count
}

#[derive(Clone, Debug)]
struct PathCount {
    path_count: i64,
    required_devices: usize,
}

fn count_paths_with_nodes<'a>(
    devices: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    required_devices: &[&str],
    path_cache: &mut HashMap<&'a str, Vec<PathCount>>,
) -> Vec<PathCount> {
    if current == target {
        return vec![PathCount {
            path_count: 1,
            required_devices: 0,
        }];
    }

    if let Some(cached) = path_cache.get(current) {
        return cached.clone();
    }

    let mut path_counts = Vec::new();

    if let Some(outputs) = devices.get(current) {
        for output in outputs {
            let sub_paths =
                count_paths_with_nodes(devices, output, target, required_devices, path_cache);
            path_counts.extend(sub_paths);
        }
    }

    // Reduce path_counts by combining those with the same required_devices count
    let mut reduced: HashMap<usize, i64> = HashMap::new();
    for pc in path_counts {
        *reduced.entry(pc.required_devices).or_insert(0) += pc.path_count;
    }

    let mut path_counts: Vec<PathCount> = reduced
        .into_iter()
        .map(|(required_devices, path_count)| PathCount {
            path_count,
            required_devices,
        })
        .collect();

    // If current is a required device, increment the required_devices count
    if required_devices.contains(&current) {
        for pc in &mut path_counts {
            pc.required_devices += 1;
        }
    }

    path_cache.insert(current, path_counts.clone());
    path_counts
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

    const SILVER_TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const GOLD_TEST_INPUT: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(SILVER_TEST_INPUT).to_string(), "5");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 11);
        assert_eq!(silver(&input).to_string(), "758");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(GOLD_TEST_INPUT).to_string(), "2");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 11);
        assert_eq!(gold(&input).to_string(), "490695961032000");
    }
}
