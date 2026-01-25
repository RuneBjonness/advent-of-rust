mod day_01;

use crate::aoc_puzzle::AocPuzzle;
use std::sync::LazyLock;

static PUZZLES: LazyLock<Vec<AocPuzzle>> = LazyLock::new(|| {
    vec![
        day_01::puzzle(),
    ]
});

pub fn puzzles() -> &'static Vec<AocPuzzle> {
    &PUZZLES
}
