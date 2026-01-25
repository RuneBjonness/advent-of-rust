mod day_01;
mod day_02;
mod day_03;

use crate::aoc_puzzle::AocPuzzle;
use std::sync::LazyLock;

static PUZZLES: LazyLock<Vec<AocPuzzle>> =
    LazyLock::new(|| vec![day_01::puzzle(), day_02::puzzle(), day_03::puzzle()]);

pub fn puzzles() -> &'static Vec<AocPuzzle> {
    &PUZZLES
}
