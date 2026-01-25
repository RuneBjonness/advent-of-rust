use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashMap;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let template = sections[0];

    let rules: Vec<(&str, &str)> = sections[1]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut counter: HashMap<char, i64> = HashMap::new();
    for c in template.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    let chars: Vec<char> = template.chars().collect();
    for i in 0..chars.len() - 1 {
        let pair = format!("{}{}", chars[i], chars[i + 1]);
        count_silver(&pair, &rules, &mut counter, 10);
    }

    let counts: Vec<i64> = counter.values().copied().collect();
    let min = *counts.iter().min().unwrap();
    let max = *counts.iter().max().unwrap();

    Box::new(max - min)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let template = sections[0];

    let rules: Vec<(&str, &str)> = sections[1]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            (parts[0], parts[1])
        })
        .collect();

    // Build a map of all unique characters that can be inserted
    let mut all_chars = std::collections::HashSet::new();
    for (_, insert) in &rules {
        all_chars.insert(insert.chars().next().unwrap());
    }

    // Initialize current_pairs and total_pairs
    let mut current_pairs: HashMap<String, HashMap<char, i64>> = HashMap::new();
    let mut total_pairs: HashMap<String, HashMap<char, i64>> = HashMap::new();
    for (pair, _) in &rules {
        let char_map: HashMap<char, i64> = all_chars.iter().map(|&c| (c, 0)).collect();
        current_pairs.insert(pair.to_string(), char_map.clone());
        total_pairs.insert(pair.to_string(), char_map);
    }

    // Compute pair expansions for 40 steps
    for _ in 0..40 {
        for (pair, _) in &rules {
            let counts = count_gold(pair, &rules, &current_pairs);
            total_pairs.insert(pair.to_string(), counts);
        }
        // Copy total_pairs to current_pairs
        for (key, value) in &total_pairs {
            current_pairs.insert(key.clone(), value.clone());
        }
    }

    // Count characters in final polymer
    let mut counter: HashMap<char, i64> = HashMap::new();
    for c in template.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    let chars: Vec<char> = template.chars().collect();
    for i in 0..chars.len() - 1 {
        let pair = format!("{}{}", chars[i], chars[i + 1]);
        if let Some(pair_counts) = total_pairs.get(&pair) {
            for (c, count) in pair_counts {
                *counter.entry(*c).or_insert(0) += count;
            }
        }
    }

    let counts: Vec<i64> = counter.values().filter(|&&v| v > 0).copied().collect();
    let min = *counts.iter().min().unwrap();
    let max = *counts.iter().max().unwrap();

    Box::new(max - min)
}

fn count_silver(pair: &str, rules: &[(&str, &str)], counter: &mut HashMap<char, i64>, steps: i32) {
    if steps == 0 {
        return;
    }

    let insert = rules
        .iter()
        .find(|(p, _)| *p == pair)
        .map(|(_, i)| i.chars().next().unwrap())
        .unwrap();

    *counter.entry(insert).or_insert(0) += 1;

    let chars: Vec<char> = pair.chars().collect();
    let pair1 = format!("{}{}", chars[0], insert);
    let pair2 = format!("{}{}", insert, chars[1]);

    count_silver(&pair1, rules, counter, steps - 1);
    count_silver(&pair2, rules, counter, steps - 1);
}

fn count_gold(
    pair: &str,
    rules: &[(&str, &str)],
    pair_counters: &HashMap<String, HashMap<char, i64>>,
) -> HashMap<char, i64> {
    let mut result: HashMap<char, i64> = HashMap::new();

    let insert = rules
        .iter()
        .find(|(p, _)| *p == pair)
        .map(|(_, i)| i.chars().next().unwrap())
        .unwrap();

    *result.entry(insert).or_insert(0) += 1;

    let chars: Vec<char> = pair.chars().collect();
    let pair1 = format!("{}{}", chars[0], insert);
    let pair2 = format!("{}{}", insert, chars[1]);

    if let Some(counts1) = pair_counters.get(&pair1) {
        for (c, count) in counts1 {
            *result.entry(*c).or_insert(0) += count;
        }
    }

    if let Some(counts2) = pair_counters.get(&pair2) {
        for (c, count) in counts2 {
            *result.entry(*c).or_insert(0) += count;
        }
    }

    result
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 14, silver, gold)
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
        let test_input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(silver(test_input).to_string(), "1588");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 14);
        assert_eq!(silver(&input).to_string(), "4244");
    }

    #[test]
    fn gold_test_input() {
        let test_input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(gold(test_input).to_string(), "2188189693529");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 14);
        assert_eq!(gold(&input).to_string(), "4807056953866");
    }
}
