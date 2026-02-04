use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    Box::new(sum_invalid_ids(input, true))
}

pub fn gold(input: &str) -> Box<dyn Display> {
    Box::new(sum_invalid_ids(input, false))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 2, silver, gold)
}

fn sum_invalid_ids(input: &str, just_two_parts: bool) -> i64 {
    input
        .split(',')
        .map(|x| {
            let parts: Vec<i64> = x.split('-').map(|s| s.parse().unwrap()).collect();
            sum_invalid_ids_in_range(parts[0], parts[1], just_two_parts)
        })
        .sum()
}

fn sum_invalid_ids_in_range(start: i64, end: i64, just_two_parts: bool) -> i64 {
    let mut sum = 0;
    let mut num_digits = (start as f64).log10().floor() as i32 + 1;
    let num_digits_end = (end as f64).log10().floor() as i32 + 1;

    while num_digits <= num_digits_end {
        let max_parts = if just_two_parts { 2 } else { num_digits };
        let mut repeated_numbers: Vec<i64> = Vec::new();

        for parts in 2..=max_parts {
            if num_digits % parts != 0 {
                continue;
            }

            let from = start.max(10_i64.pow(num_digits as u32 - 1));
            let to = end.min(10_i64.pow(num_digits as u32) - 1);

            let num_digits_per_part = num_digits / parts;
            let divisor = 10_i64.pow((num_digits - num_digits_per_part) as u32);

            let first_part_start = from / divisor;
            let first_part_end = to / divisor;

            for first_part in first_part_start..=first_part_end {
                let repeated_number = construct_number(first_part, parts);
                if repeated_number >= from && repeated_number <= to {
                    if max_parts > 2 && repeated_numbers.contains(&repeated_number) {
                        continue;
                    }
                    repeated_numbers.push(repeated_number);
                    sum += repeated_number;
                }
            }
        }
        num_digits += 1;
    }
    sum
}

fn construct_number(repeated_part: i64, count: i32) -> i64 {
    let mut number = 0;
    let mut multiplier = 1;
    let base_multiplier = 10_i64.pow(((repeated_part as f64).log10().floor() as u32) + 1);

    for _ in 0..count {
        number += repeated_part * multiplier;
        multiplier *= base_multiplier;
    }
    number
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
        assert_eq!(silver("10-12").to_string(), "11");
    }

    #[test]
    fn silver_test_input_extended() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";
        assert_eq!(silver(input).to_string(), "1227775554");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 2);
        assert_eq!(silver(&input).to_string(), "24043483400");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("10-12").to_string(), "11");
    }

    #[test]
    fn gold_test_input_extended() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";
        assert_eq!(gold(input).to_string(), "4174379265");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 2);
        assert_eq!(gold(&input).to_string(), "38262920235");
    }
}
