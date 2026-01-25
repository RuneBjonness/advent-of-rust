use crate::aoc_puzzle::AocPuzzle;
use crate::y2015;

pub fn get_puzzles(year: Option<u16>, day: Option<u8>) -> Vec<&'static AocPuzzle> {
    let all_puzzles: &[AocPuzzle] = y2015::puzzles();

    all_puzzles
        .iter()
        .filter(|puzzle| year.map_or(true, |y| puzzle.year == y))
        .filter(|puzzle| day.map_or(true, |d| puzzle.day == d))
        .collect()
}
