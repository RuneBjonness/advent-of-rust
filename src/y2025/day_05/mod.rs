use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let range_section = sections[0];
    let id_section = sections[1];

    let mut ranges: Vec<[i64; 2]> = range_section
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split('-').map(|s| s.parse().unwrap()).collect();
            [parts[0], parts[1]]
        })
        .collect();
    ranges.sort_by_key(|r| r[0]);

    let mut ids: Vec<i64> = id_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    ids.sort();

    let mut range_index = 0;
    let mut count = 0;

    for id in ids {
        if id >= ranges[range_index][0] && id <= ranges[range_index][1] {
            count += 1;
        } else {
            while range_index < ranges.len() - 1 && id > ranges[range_index][1] {
                range_index += 1;
            }
            if id >= ranges[range_index][0] && id <= ranges[range_index][1] {
                count += 1;
            }
        }
    }

    Box::new(count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let range_section = input.split("\n\n").next().unwrap();

    let mut ranges: Vec<[i64; 2]> = range_section
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split('-').map(|s| s.parse().unwrap()).collect();
            [parts[0], parts[1]]
        })
        .collect();
    ranges.sort_by_key(|r| r[0]);

    let mut count = 0;
    let mut last_max = -1;

    for [min, max] in ranges {
        if min > last_max {
            count += max - min + 1;
            last_max = max;
        } else if max > last_max {
            count += max - last_max;
            last_max = max;
        }
    }

    Box::new(count)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let range_section = sections[0];
    let id_section = sections[1];

    let mut ranges: Vec<[i64; 2]> = range_section
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split('-').map(|s| s.parse().unwrap()).collect();
            [parts[0], parts[1]]
        })
        .collect();
    ranges.sort_by_key(|r| r[0]);

    let mut ids: Vec<i64> = id_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    ids.sort();

    let mut range_index = 0;
    let mut silver_count = 0;
    let mut gold_count = ranges[range_index][1] - ranges[range_index][0] + 1;
    let mut last_max = ranges[range_index][1];

    for id in ids {
        if id >= ranges[range_index][0] && id <= ranges[range_index][1] {
            silver_count += 1;
        } else {
            while range_index < ranges.len() - 1 && id > ranges[range_index][1] {
                range_index += 1;
                if ranges[range_index][0] > last_max {
                    gold_count += ranges[range_index][1] - ranges[range_index][0] + 1;
                    last_max = ranges[range_index][1];
                } else if ranges[range_index][1] > last_max {
                    gold_count += ranges[range_index][1] - last_max;
                    last_max = ranges[range_index][1];
                }
            }
            if id >= ranges[range_index][0] && id <= ranges[range_index][1] {
                silver_count += 1;
            }
        }
    }

    (Box::new(silver_count), Box::new(gold_count))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 5, silver, gold).with_both(both)
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

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "3");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 5);
        assert_eq!(silver(&input).to_string(), "615");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "14");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 5);
        assert_eq!(gold(&input).to_string(), "353716783056994");
    }
}
