use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode: {}", value),
        }
    }
}

struct Instruction {
    opcode: i32,
    param1_mode: ParameterMode,
    param2_mode: ParameterMode,
    param3_mode: ParameterMode,
}

fn get_instruction(instruction: i32) -> Instruction {
    let opcode = instruction % 100;
    let param1_mode = ((instruction / 100) % 10).into();
    let param2_mode = ((instruction / 1000) % 10).into();
    let param3_mode = ((instruction / 10000) % 10).into();
    Instruction {
        opcode,
        param1_mode,
        param2_mode,
        param3_mode,
    }
}

fn get_program_value(p: &[i32], index: usize, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => p[p[index] as usize],
        ParameterMode::Immediate => p[index],
    }
}

fn set_program_value(p: &mut [i32], index: usize, mode: ParameterMode, value: i32) {
    match mode {
        ParameterMode::Position => {
            let addr = p[index] as usize;
            p[addr] = value;
        }
        ParameterMode::Immediate => {
            panic!("Immediate mode not supported for write operations");
        }
    }
}

fn run_intcode(p: &mut [i32], input: i32) -> Vec<i32> {
    let mut outputs = Vec::new();
    let mut i = 0;
    let mut instruction = get_instruction(p[i]);

    while instruction.opcode != 99 {
        match instruction.opcode {
            1 => {
                // Add
                let val = get_program_value(p, i + 1, instruction.param1_mode)
                    + get_program_value(p, i + 2, instruction.param2_mode);
                set_program_value(p, i + 3, instruction.param3_mode, val);
                i += 4;
            }
            2 => {
                // Multiply
                let val = get_program_value(p, i + 1, instruction.param1_mode)
                    * get_program_value(p, i + 2, instruction.param2_mode);
                set_program_value(p, i + 3, instruction.param3_mode, val);
                i += 4;
            }
            3 => {
                // Input
                set_program_value(p, i + 1, instruction.param1_mode, input);
                i += 2;
            }
            4 => {
                // Output
                outputs.push(get_program_value(p, i + 1, instruction.param1_mode));
                i += 2;
            }
            5 => {
                // Jump-if-true
                if get_program_value(p, i + 1, instruction.param1_mode) != 0 {
                    i = get_program_value(p, i + 2, instruction.param2_mode) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                // Jump-if-false
                if get_program_value(p, i + 1, instruction.param1_mode) == 0 {
                    i = get_program_value(p, i + 2, instruction.param2_mode) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                // Less than
                let val = if get_program_value(p, i + 1, instruction.param1_mode)
                    < get_program_value(p, i + 2, instruction.param2_mode)
                {
                    1
                } else {
                    0
                };
                set_program_value(p, i + 3, instruction.param3_mode, val);
                i += 4;
            }
            8 => {
                // Equals
                let val = if get_program_value(p, i + 1, instruction.param1_mode)
                    == get_program_value(p, i + 2, instruction.param2_mode)
                {
                    1
                } else {
                    0
                };
                set_program_value(p, i + 3, instruction.param3_mode, val);
                i += 4;
            }
            _ => panic!("Unknown opcode: {}", instruction.opcode),
        }
        instruction = get_instruction(p[i]);
    }
    outputs
}

pub fn silver(input: &str) -> Box<dyn Display> {
    let mut program: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let outputs = run_intcode(&mut program, 1);
    Box::new(outputs[outputs.len() - 1])
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let mut program: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let outputs = run_intcode(&mut program, 5);
    Box::new(outputs[outputs.len() - 1])
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2019, 5, silver, gold)
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
    fn test_parameter_modes() {
        // Test immediate mode: 1002,4,3,4,33 multiplies 33 by 3 and stores at position 4
        let mut program = vec![1002, 4, 3, 4, 33];
        run_intcode(&mut program, 0);
        assert_eq!(program[4], 99);
    }

    #[test]
    fn test_equals_position_mode() {
        // Using position mode, equal to 8
        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let outputs = run_intcode(&mut program, 8);
        assert_eq!(outputs[0], 1);

        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let outputs = run_intcode(&mut program, 7);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn test_less_than_position_mode() {
        // Using position mode, less than 8
        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let outputs = run_intcode(&mut program, 7);
        assert_eq!(outputs[0], 1);

        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let outputs = run_intcode(&mut program, 8);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn test_equals_immediate_mode() {
        // Using immediate mode, equal to 8
        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let outputs = run_intcode(&mut program, 8);
        assert_eq!(outputs[0], 1);

        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let outputs = run_intcode(&mut program, 7);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn test_less_than_immediate_mode() {
        // Using immediate mode, less than 8
        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let outputs = run_intcode(&mut program, 7);
        assert_eq!(outputs[0], 1);

        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let outputs = run_intcode(&mut program, 8);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn test_jump_position_mode() {
        // Position mode jump test - output 0 if input is 0, 1 otherwise
        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let outputs = run_intcode(&mut program, 0);
        assert_eq!(outputs[0], 0);

        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let outputs = run_intcode(&mut program, 5);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn test_jump_immediate_mode() {
        // Immediate mode jump test - output 0 if input is 0, 1 otherwise
        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let outputs = run_intcode(&mut program, 0);
        assert_eq!(outputs[0], 0);

        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let outputs = run_intcode(&mut program, 5);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn test_larger_example() {
        // Output 999 if input < 8, 1000 if input == 8, 1001 if input > 8
        let program_str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut program: Vec<i32> = program_str.split(',').map(|x| x.parse().unwrap()).collect();
        let outputs = run_intcode(&mut program, 7);
        assert_eq!(outputs[0], 999);

        let mut program: Vec<i32> = program_str.split(',').map(|x| x.parse().unwrap()).collect();
        let outputs = run_intcode(&mut program, 8);
        assert_eq!(outputs[0], 1000);

        let mut program: Vec<i32> = program_str.split(',').map(|x| x.parse().unwrap()).collect();
        let outputs = run_intcode(&mut program, 9);
        assert_eq!(outputs[0], 1001);
    }

    #[test]
    fn silver_test_input() {
        assert_eq!(silver("3,0,4,0,99").to_string(), "1");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2019, 5);
        assert_eq!(silver(&input).to_string(), "4511442");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2019, 5);
        assert_eq!(gold(&input).to_string(), "12648139");
    }
}
