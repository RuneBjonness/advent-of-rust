use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(' ').filter(|x| !x.is_empty()).collect())
        .collect();

    let value_rows = &lines[..lines.len() - 1];
    let operation_row = &lines[lines.len() - 1];

    let mut result = 0;
    for col in 0..lines[0].len() {
        let values: Vec<i64> = value_rows
            .iter()
            .map(|row| row[col].parse::<i64>().unwrap())
            .collect();

        if operation_row[col] == "+" {
            result += values.iter().sum::<i64>();
        } else {
            result += values.iter().product::<i64>();
        }
    }

    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let value_rows = &grid[..grid.len() - 1];
    let operation_row = &grid[grid.len() - 1];

    let mut result = 0i64;
    let mut part_result = 0i64;
    let mut operator = '+';

    let max_col = value_rows.iter().map(|r| r.len()).max().unwrap_or(0);

    for col in 0..max_col {
        let value_str: String = value_rows
            .iter()
            .map(|row| row.get(col).copied().unwrap_or(' '))
            .collect();
        let value_str_trimmed = value_str.replace(' ', "");
        let value = if value_str_trimmed.is_empty() {
            0
        } else {
            value_str_trimmed.parse::<i64>().unwrap()
        };

        let op_char = operation_row.get(col).copied().unwrap_or(' ');

        if op_char == '+' {
            operator = '+';
            result += part_result;
            part_result = value;
        } else if op_char == '*' {
            operator = '*';
            result += part_result;
            part_result = value;
        } else if operator == '+' {
            part_result += value;
        } else if operator == '*' && value != 0 {
            part_result *= value;
        }

        if col == max_col - 1 {
            result += part_result;
        }
    }

    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 6, silver, gold)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(year: u16, day: u8) -> String {
        fs::read_to_string(format!("./input/{}_{:02}.txt", year, day))
            .unwrap()
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .to_string()
    }

    #[test]
    fn silver_test_input() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(silver(input).to_string(), "4277556");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 6);
        assert_eq!(silver(&input).to_string(), "4405895212738");
    }

    #[test]
    fn gold_test_input() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(gold(input).to_string(), "3263827");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 6);
        assert_eq!(gold(&input).to_string(), "7450962489289");
    }
}
