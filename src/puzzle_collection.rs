use crate::aoc_puzzle::AocPuzzle;
use crate::{y2015, y2016, y2017, y2018, y2019, y2020, y2021, y2022, y2023, y2024, y2025};

pub fn get_puzzles(year: Option<u16>, day: Option<u8>) -> Vec<&'static AocPuzzle> {
    let iter: Box<dyn Iterator<Item = &'static AocPuzzle>> = match year {
        Some(2015) => Box::new(y2015::puzzles().iter()),
        Some(2016) => Box::new(y2016::puzzles().iter()),
        Some(2017) => Box::new(y2017::puzzles().iter()),
        Some(2018) => Box::new(y2018::puzzles().iter()),
        Some(2019) => Box::new(y2019::puzzles().iter()),
        Some(2020) => Box::new(y2020::puzzles().iter()),
        Some(2021) => Box::new(y2021::puzzles().iter()),
        Some(2022) => Box::new(y2022::puzzles().iter()),
        Some(2023) => Box::new(y2023::puzzles().iter()),
        Some(2024) => Box::new(y2024::puzzles().iter()),
        Some(2025) => Box::new(y2025::puzzles().iter()),
        Some(_) => return Vec::new(),
        None => Box::new(
            y2015::puzzles()
                .iter()
                .chain(y2016::puzzles().iter())
                .chain(y2017::puzzles().iter())
                .chain(y2018::puzzles().iter())
                .chain(y2019::puzzles().iter())
                .chain(y2020::puzzles().iter())
                .chain(y2021::puzzles().iter())
                .chain(y2022::puzzles().iter())
                .chain(y2023::puzzles().iter())
                .chain(y2024::puzzles().iter())
                .chain(y2025::puzzles().iter()),
        ),
    };

    match day {
        Some(d) => iter.filter(|p| p.day == d).collect(),
        None => iter.collect(),
    }
}
