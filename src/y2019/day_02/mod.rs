use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut program: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    let result = run_intcode(&mut program);
    Box::new(result[0])
}

pub fn gold(input: &str) -> Box<dyn Display> {
    for i in 0..100 {
        for j in 0..100 {
            let mut program: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
            program[1] = i;
            program[2] = j;
            if run_intcode(&mut program)[0] == 19690720 {
                return Box::new(100 * i + j);
            }
        }
    }
    Box::new(0)
}

fn run_intcode(p: &mut [usize]) -> &[usize] {
    let mut i = 0;
    while p[i] != 99 {
        if p[i] == 1 {
            let a = p[p[i + 1]];
            let b = p[p[i + 2]];
            let dest = p[i + 3];
            p[dest] = a + b;
        } else if p[i] == 2 {
            let a = p[p[i + 1]];
            let b = p[p[i + 2]];
            let dest = p[i + 3];
            p[dest] = a * b;
        }
        i += 4;
    }
    p
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2019, 2, silver, gold)
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
    fn test_run_intcode() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run_intcode(&mut program);
        assert_eq!(result, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2019, 2);
        assert_eq!(silver(&input).to_string(), "4138658");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2019, 2);
        assert_eq!(gold(&input).to_string(), "7264");
    }
}
