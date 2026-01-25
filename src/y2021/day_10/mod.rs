use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut error_score = 0;

    for line in input.lines() {
        let mut openings = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        for ch in chars {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                openings.push(ch);
            } else {
                let last_opening = openings.pop();
                if last_opening != Some('(') && ch == ')' {
                    error_score += 3;
                    break;
                } else if last_opening != Some('[') && ch == ']' {
                    error_score += 57;
                    break;
                } else if last_opening != Some('{') && ch == '}' {
                    error_score += 1197;
                    break;
                } else if last_opening != Some('<') && ch == '>' {
                    error_score += 25137;
                    break;
                }
            }
        }
    }

    Box::new(error_score)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut autocomplete_scores = Vec::new();

    for line in input.lines() {
        let mut is_corrupt = false;
        let mut openings = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        for ch in chars {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                openings.push(ch);
            } else {
                let last_opening = openings.pop();
                if (last_opening != Some('(') && ch == ')')
                    || (last_opening != Some('[') && ch == ']')
                    || (last_opening != Some('{') && ch == '}')
                    || (last_opening != Some('<') && ch == '>')
                {
                    is_corrupt = true;
                    break;
                }
            }
        }

        if !is_corrupt {
            let mut total_score: i64 = 0;
            for ch in openings.iter().rev() {
                total_score *= 5;
                match ch {
                    '(' => total_score += 1,
                    '[' => total_score += 2,
                    '{' => total_score += 3,
                    '<' => total_score += 4,
                    _ => {}
                }
            }
            autocomplete_scores.push(total_score);
        }
    }

    autocomplete_scores.sort();
    let result = autocomplete_scores[autocomplete_scores.len() / 2];
    Box::new(result)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 10, silver, gold)
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

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "26397");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 10);
        assert_eq!(silver(&input).to_string(), "299793");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "288957");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 10);
        assert_eq!(gold(&input).to_string(), "3654963618");
    }
}
