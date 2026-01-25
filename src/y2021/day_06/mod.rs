use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut timers: Vec<u8> = input.split(',').map(|s| s.parse().unwrap()).collect();

    for _t in 0..80 {
        let mut offspring = Vec::new();
        for timer in timers.iter_mut() {
            if *timer == 0 {
                *timer = 6;
                offspring.push(8);
            } else {
                *timer -= 1;
            }
        }
        timers.extend(offspring);
    }

    Box::new(timers.len())
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let timers: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    // Count fish by timer value (0-8)
    let mut fish = [0u64; 9];
    fish[1] = timers.iter().filter(|&&x| x == 1).count() as u64;
    fish[2] = timers.iter().filter(|&&x| x == 2).count() as u64;
    fish[3] = timers.iter().filter(|&&x| x == 3).count() as u64;
    fish[4] = timers.iter().filter(|&&x| x == 4).count() as u64;
    fish[5] = timers.iter().filter(|&&x| x == 5).count() as u64;

    for _t in 0..256 {
        let gen1 = fish[0];
        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7] + gen1;
        fish[7] = fish[8];
        fish[8] = gen1;
    }

    Box::new(fish.iter().sum::<u64>())
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 6, silver, gold)
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
        assert_eq!(silver("3,4,3,1,2").to_string(), "5934");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 6);
        assert_eq!(silver(&input).to_string(), "373378");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("3,4,3,1,2").to_string(), "26984457539");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 6);
        assert_eq!(gold(&input).to_string(), "1682576647495");
    }
}
