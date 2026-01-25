use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let numbers: Vec<&str> = input.lines().collect();
    let bit_length = numbers[0].len();
    let mut bit_count = vec![0; bit_length];
    let mut digit_count = 0;

    for number in &numbers {
        digit_count += 1;
        for (idx, bit) in number.chars().enumerate() {
            if bit == '1' {
                bit_count[idx] += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for count in bit_count {
        if count > digit_count - count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_value = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_value = i32::from_str_radix(&epsilon, 2).unwrap();

    Box::new(gamma_value * epsilon_value)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let digits: Vec<&str> = input.lines().collect();
    let bit_length = digits[0].len();

    let mut oxygen_rating = digits.clone();
    let mut co2_rating = digits.clone();

    for i in 0..bit_length {
        if oxygen_rating.len() > 1 {
            let count = oxygen_rating
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .count();
            let filter_char = if count >= oxygen_rating.len() - count {
                '1'
            } else {
                '0'
            };
            oxygen_rating.retain(|v| v.chars().nth(i).unwrap() == filter_char);
        }
    }

    for i in 0..bit_length {
        if co2_rating.len() > 1 {
            let count = co2_rating
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .count();
            let filter_char = if count >= co2_rating.len() - count {
                '0'
            } else {
                '1'
            };
            co2_rating.retain(|v| v.chars().nth(i).unwrap() == filter_char);
        }
    }

    let oxygen_value = i32::from_str_radix(oxygen_rating[0], 2).unwrap();
    let co2_value = i32::from_str_radix(co2_rating[0], 2).unwrap();

    Box::new(oxygen_value * co2_value)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 3, silver, gold)
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
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(silver(input).to_string(), "198");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 3);
        assert_eq!(silver(&input).to_string(), "4139586");
    }

    #[test]
    fn gold_test_input() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(gold(input).to_string(), "230");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 3);
        assert_eq!(gold(&input).to_string(), "1800151");
    }
}
