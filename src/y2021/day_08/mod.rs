use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let count = input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|d| {
                    let len = d.len();
                    len == 2 || len == 3 || len == 4 || len == 7
                })
                .count()
        })
        .sum::<usize>();

    Box::new(count)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let sum: usize = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" | ").collect();
            let digits: Vec<&str> = parts[0].split_whitespace().collect();

            // Find known patterns
            let cf = digits.iter().find(|d| d.len() == 2).unwrap();
            let acf = digits.iter().find(|d| d.len() == 3).unwrap();
            let bcdf = digits.iter().find(|d| d.len() == 4).unwrap();

            // Deduce the segment mapping (store as chars)
            let mut config = vec![' '; 7];

            // Determine segments c (index 2) and f (index 5)
            let cf_chars: Vec<char> = cf.chars().collect();
            if digits.iter().filter(|d| d.contains(cf_chars[0])).count() == 9 {
                config[2] = cf_chars[1];
                config[5] = cf_chars[0];
            } else {
                config[2] = cf_chars[0];
                config[5] = cf_chars[1];
            }

            // Determine segment a (index 0)
            config[0] = acf.chars().find(|c| !cf.contains(*c)).unwrap();

            // Determine segments b (index 1) and d (index 3)
            let bd: Vec<char> = bcdf.chars().filter(|c| !cf.contains(*c)).collect();
            if digits.iter().filter(|d| d.contains(bd[0])).count() == 6 {
                config[1] = bd[0];
                config[3] = bd[1];
            } else {
                config[1] = bd[1];
                config[3] = bd[0];
            }

            // Determine segments e (index 4) and g (index 6)
            let eg: Vec<char> = "abcdefg".chars().filter(|c| !config.contains(c)).collect();
            if digits.iter().filter(|d| d.contains(eg[0])).count() == 4 {
                config[4] = eg[0];
                config[6] = eg[1];
            } else {
                config[4] = eg[1];
                config[6] = eg[0];
            }

            // Build digit patterns (sorted)
            let digit_patterns: Vec<String> = vec![
                vec![
                    config[0], config[1], config[2], config[4], config[5], config[6],
                ],
                vec![config[2], config[5]],
                vec![config[0], config[2], config[3], config[4], config[6]],
                vec![config[0], config[2], config[3], config[5], config[6]],
                vec![config[1], config[2], config[3], config[5]],
                vec![config[0], config[1], config[3], config[5], config[6]],
                vec![
                    config[0], config[1], config[3], config[4], config[5], config[6],
                ],
                vec![config[0], config[2], config[5]],
                vec![
                    config[0], config[1], config[2], config[3], config[4], config[5], config[6],
                ],
                vec![
                    config[0], config[1], config[2], config[3], config[5], config[6],
                ],
            ]
            .iter()
            .map(|chars| {
                let mut sorted = chars.clone();
                sorted.sort_unstable();
                sorted.into_iter().collect()
            })
            .collect();

            // Decode output digits
            let number: usize = parts[1]
                .split_whitespace()
                .rev()
                .enumerate()
                .map(|(i, x)| {
                    let mut chars: Vec<char> = x.chars().collect();
                    chars.sort_unstable();
                    let sorted: String = chars.into_iter().collect();
                    let digit = digit_patterns.iter().position(|s| s == &sorted).unwrap();
                    10_usize.pow(i as u32) * digit
                })
                .sum();

            number
        })
        .sum();

    Box::new(sum)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 8, silver, gold)
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
        let test_input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(silver(test_input).to_string(), "26");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 8);
        assert_eq!(silver(&input).to_string(), "303");
    }

    #[test]
    fn gold_test_input() {
        let test_input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(gold(test_input).to_string(), "61229");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 8);
        assert_eq!(gold(&input).to_string(), "961734");
    }
}
