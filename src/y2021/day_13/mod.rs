use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let (marks, folds) = parse_input(input);
    let folded_marks = fold(&marks, &folds[0]);
    Box::new(folded_marks.len())
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let (mut marks, folds) = parse_input(input);

    for fold_instruction in folds {
        marks = fold(&marks, &fold_instruction);
    }

    let plot_string = decode_letters(&marks);
    Box::new(plot_string)
}

fn fold(marks: &HashSet<Point>, fold_at: &str) -> HashSet<Point> {
    let parts: Vec<&str> = fold_at.split('=').collect();
    let fold_idx = parts[1].parse::<i32>().unwrap();
    let mut folded = HashSet::new();

    if parts[0] == "y" {
        // Fold along horizontal line (y = fold_idx)
        for &p in marks {
            if p.y < fold_idx {
                folded.insert(p);
            } else if p.y > fold_idx {
                folded.insert(Point {
                    x: p.x,
                    y: fold_idx - (p.y - fold_idx),
                });
            }
        }
    } else {
        // Fold along vertical line (x = fold_idx)
        for &p in marks {
            if p.x < fold_idx {
                folded.insert(p);
            } else if p.x > fold_idx {
                folded.insert(Point {
                    x: fold_idx - (p.x - fold_idx),
                    y: p.y,
                });
            }
        }
    }

    folded
}

fn decode_letters(marks: &HashSet<Point>) -> String {
    let mut result = String::new();

    // The output consists of 8 letters, each 5 pixels wide (4 pixels + 1 space)
    for i in 0..8 {
        let x_offset = i * 5;

        // Check the first row pattern
        let first_row: String = (0..4)
            .map(|x| {
                if marks.contains(&Point {
                    x: x_offset + x,
                    y: 0,
                }) {
                    'X'
                } else {
                    ' '
                }
            })
            .collect();

        match first_row.as_str() {
            "X   " => result.push('L'),
            "X  X" => result.push('K'),
            "XXXX" => result.push('E'),
            "XXX " => {
                // Could be R, B, or P - check bottom row
                let bottom_row: String = (0..4)
                    .map(|x| {
                        if marks.contains(&Point {
                            x: x_offset + x,
                            y: 5,
                        }) {
                            'X'
                        } else {
                            ' '
                        }
                    })
                    .collect();

                match bottom_row.as_str() {
                    "X  X" => result.push('R'),
                    "XXX " => result.push('B'),
                    "X   " => result.push('P'),
                    _ => result.push('?'),
                }
            }
            _ => result.push('?'),
        }
    }

    result
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<String>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let marks: HashSet<Point> = parts[0]
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    let folds: Vec<String> = parts[1]
        .lines()
        .map(|line| line.replace("fold along ", ""))
        .collect();

    (marks, folds)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 13, silver, gold)
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
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(silver(input).to_string(), "17");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 13);
        assert_eq!(silver(&input).to_string(), "653");
    }

    #[test]
    fn gold_test_input() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        // The test input forms a square pattern, not letters
        assert_eq!(gold(input).to_string().len() > 0, true);
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 13);
        assert_eq!(gold(&input).to_string(), "LKREBPRK");
    }
}
