use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let (mut boards, numbers) = parse_input(input);
    let (winning_board, num) = mark_and_test_boards(&mut boards, &numbers);
    Box::new(score_board(&winning_board, num))
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let (boards, numbers) = parse_input(input);
    let (winning_board, num) = get_last_winning_board(boards, &numbers);
    Box::new(score_board(&winning_board, num))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 4, silver, gold)
}

fn parse_input(input: &str) -> (Vec<Vec<Vec<i32>>>, Vec<i32>) {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let numbers: Vec<i32> = blocks[0].split(',').map(|n| n.parse().unwrap()).collect();

    let boards: Vec<Vec<Vec<i32>>> = blocks[1..]
        .iter()
        .map(|b| {
            b.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    (boards, numbers)
}

fn get_last_winning_board(mut boards: Vec<Vec<Vec<i32>>>, numbers: &[i32]) -> (Vec<Vec<i32>>, i32) {
    for n in 0..numbers.len() {
        // Mark all boards with current number
        for b in 0..boards.len() {
            for r in 0..boards[b].len() {
                for c in 0..boards[b][r].len() {
                    if boards[b][r][c] == numbers[n] {
                        boards[b][r][c] = -1;
                    }
                }
            }
        }

        // Remove winning boards
        boards.retain(|board| !test_board(board));

        // When only one board remains, play it until it wins
        if boards.len() == 1 {
            return mark_and_test_boards(&mut boards, &numbers[n..]);
        }
    }
    (boards[0].clone(), 0)
}

fn mark_and_test_boards(boards: &mut [Vec<Vec<i32>>], numbers: &[i32]) -> (Vec<Vec<i32>>, i32) {
    for &num in numbers {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell == num {
                        *cell = -1;
                    }
                }
            }
            if test_board(board) {
                return (board.clone(), num);
            }
        }
    }
    (boards[0].clone(), 0)
}

fn test_board(board: &[Vec<i32>]) -> bool {
    // Check rows
    if board.iter().any(|row| row.iter().sum::<i32>() == -5) {
        return true;
    }

    // Check columns
    for c in 0..5 {
        if board.iter().map(|row| row[c]).sum::<i32>() == -5 {
            return true;
        }
    }

    false
}

fn score_board(board: &[Vec<i32>], last_num: i32) -> i32 {
    board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&n| n > 0)
        .sum::<i32>()
        * last_num
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
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(silver(input).to_string(), "4512");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 4);
        assert_eq!(silver(&input).to_string(), "51034");
    }

    #[test]
    fn gold_test_input() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(gold(input).to_string(), "1924");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 4);
        assert_eq!(gold(&input).to_string(), "5434");
    }
}
