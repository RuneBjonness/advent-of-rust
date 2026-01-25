use crate::aoc_puzzle::AocPuzzle;
use crate::{y2015, y2016, y2017, y2018, y2019};

pub fn get_puzzles(year: Option<u16>, day: Option<u8>) -> Vec<&'static AocPuzzle> {
    let mut all_puzzles: Vec<&AocPuzzle> = Vec::new();
    all_puzzles.extend(y2015::puzzles().iter());
    all_puzzles.extend(y2016::puzzles().iter());
    all_puzzles.extend(y2017::puzzles().iter());
    all_puzzles.extend(y2018::puzzles().iter());
    all_puzzles.extend(y2019::puzzles().iter());

    all_puzzles
        .into_iter()
        .filter(|puzzle| year.map_or(true, |y| puzzle.year == y))
        .filter(|puzzle| day.map_or(true, |d| puzzle.day == d))
        .collect()
}
