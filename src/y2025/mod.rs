mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

use crate::aoc_puzzle::AocPuzzle;
use std::sync::LazyLock;

static PUZZLES: LazyLock<Vec<AocPuzzle>> = LazyLock::new(|| {
    vec![
        day_01::puzzle(),
        day_02::puzzle(),
        day_03::puzzle(),
        day_04::puzzle(),
        day_05::puzzle(),
        day_06::puzzle(),
        day_07::puzzle(),
        day_08::puzzle(),
    ]
});

pub fn puzzles() -> &'static Vec<AocPuzzle> {
    &PUZZLES
}
