use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let numbers: Vec<&str> = input.lines().collect();
    let mut sum = numbers[0].to_string();
    for i in 1..numbers.len() {
        sum = add(&sum, numbers[i]);
    }
    Box::new(magnitude(&parse_pairs(&sum)))
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let numbers: Vec<&str> = input.lines().collect();
    let mut sums = Vec::new();
    for i in 1..numbers.len() {
        for j in 0..i {
            sums.push(add(numbers[j], numbers[i]));
            sums.push(add(numbers[i], numbers[j]));
        }
    }
    let max_magnitude = sums
        .iter()
        .map(|s| magnitude(&parse_pairs(s)))
        .max()
        .unwrap();
    Box::new(max_magnitude)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 18, silver, gold)
}

fn add(a: &str, b: &str) -> String {
    reduce(&format!("[{},{}]", a, b))
}

fn reduce(a: &str) -> String {
    let mut result = a.to_string();
    loop {
        if let Some(r) = explode(&result) {
            result = r;
            continue;
        }
        if let Some(r) = split(&result) {
            result = r;
            continue;
        }
        return result;
    }
}

fn explode(n: &str) -> Option<String> {
    let mut lvl = 0;
    let chars: Vec<char> = n.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == '[' {
            lvl += 1;
            if lvl == 5 {
                let exp_idx_end = n[i + 1..].find(']').unwrap() + i + 1;
                let pair: Vec<&str> = n[i + 1..exp_idx_end].split(',').collect();
                let left = add_to_last_number(&n[..i], pair[0]);
                let right = add_to_first_number(&n[exp_idx_end + 1..], pair[1]);
                return Some(format!("{}0{}", left, right));
            }
        } else if chars[i] == ']' {
            lvl -= 1;
        }
    }
    None
}

fn add_to_last_number(s: &str, add: &str) -> String {
    let mut number = String::new();
    let chars: Vec<char> = s.chars().collect();
    for i in (0..chars.len()).rev() {
        if chars[i].is_ascii_digit() {
            number.insert(0, chars[i]);
        } else if !number.is_empty() {
            let sum = add.parse::<i32>().unwrap() + number.parse::<i32>().unwrap();
            return format!("{}{}{}", &s[..i + 1], sum, &s[i + 1 + number.len()..]);
        }
    }
    s.to_string()
}

fn add_to_first_number(s: &str, add: &str) -> String {
    let mut number = String::new();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_ascii_digit() {
            number.push(chars[i]);
        } else if number.len() > 0 {
            let sum = add.parse::<i32>().unwrap() + number.parse::<i32>().unwrap();
            return format!("{}{}{}", &s[..i - number.len()], sum, &s[i..]);
        }
    }
    s.to_string()
}

fn split(n: &str) -> Option<String> {
    let mut number = String::new();
    let chars: Vec<char> = n.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_ascii_digit() {
            number.push(chars[i]);
        } else if number.len() > 1 {
            let num = number.parse::<i32>().unwrap();
            return Some(format!(
                "{}[{},{}]{}",
                &n[..i - number.len()],
                num / 2,
                (num + 1) / 2,
                &n[i..]
            ));
        } else {
            number.clear();
        }
    }
    None
}

#[derive(Debug)]
struct Pair {
    left: Option<Box<Pair>>,
    left_val: Option<i32>,
    right: Option<Box<Pair>>,
    right_val: Option<i32>,
}

fn magnitude(p: &Pair) -> i32 {
    let l = p
        .left_val
        .unwrap_or_else(|| magnitude(p.left.as_ref().unwrap()));
    let r = p
        .right_val
        .unwrap_or_else(|| magnitude(p.right.as_ref().unwrap()));
    3 * l + 2 * r
}

fn parse_pairs(n: &str) -> Pair {
    let mut p = Pair {
        left: None,
        left_val: None,
        right: None,
        right_val: None,
    };
    let chars: Vec<char> = n.chars().collect();

    let r_idx = if chars[1] == '[' {
        let idx = get_closing_index(n, 1) + 1;
        p.left = Some(Box::new(parse_pairs(&n[1..idx])));
        idx
    } else {
        let comma_idx = n.find(',').unwrap();
        p.left_val = Some(n[1..comma_idx].parse().unwrap());
        comma_idx
    };

    let r_idx = r_idx + 1;
    if chars[r_idx] == '[' {
        p.right = Some(Box::new(parse_pairs(&n[r_idx..n.len() - 1])));
    } else {
        p.right_val = Some(n[r_idx..n.len() - 1].parse().unwrap());
    }

    p
}

fn get_closing_index(n: &str, start: usize) -> usize {
    let mut lvl = 1;
    let chars: Vec<char> = n.chars().collect();
    for i in start + 1..chars.len() {
        if chars[i] == '[' {
            lvl += 1;
        } else if chars[i] == ']' {
            lvl -= 1;
            if lvl == 0 {
                return i;
            }
        }
    }
    n.len()
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
    fn test_explode() {
        assert_eq!(
            explode("[[[[[9,8],1],2],3],4]"),
            Some("[[[[0,9],2],3],4]".to_string())
        );
        assert_eq!(
            explode("[7,[6,[5,[4,[3,2]]]]]"),
            Some("[7,[6,[5,[7,0]]]]".to_string())
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            split("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
            Some("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string())
        );
    }

    #[test]
    fn test_add() {
        let result = add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]");
        assert_eq!(result, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn silver_test_input() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(silver(input).to_string(), "4140");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 18);
        assert_eq!(silver(&input).to_string(), "3884");
    }

    #[test]
    fn gold_test_input() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(gold(input).to_string(), "3993");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 18);
        assert_eq!(gold(&input).to_string(), "4595");
    }
}
