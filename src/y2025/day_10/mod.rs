use crate::aoc_puzzle::AocPuzzle;
use std::collections::HashMap;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let machines: Vec<Vec<&str>> = input.lines().map(|m| m.split(' ').collect()).collect();
    let mut result = 0;
    for machine in machines {
        result += calculate_minimum_button_presses_for_target(
            &get_target(&machine),
            &get_buttons(&machine),
        );
    }
    Box::new(result)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let machines: Vec<Vec<&str>> = input.lines().map(|m| m.split(' ').collect()).collect();
    let mut result = 0;
    for machine in machines {
        let target = get_joltage_target(&machine);
        let combinations = calculate_button_combinations(&get_buttons(&machine), get_target(&machine).len());
        let min_buttons = minimum_buttons_to_joltage_target(&target, &combinations, &mut HashMap::new());
        result += min_buttons;
    }
    Box::new(result)
}

pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    let machines: Vec<Vec<&str>> = input.lines().map(|m| m.split(' ').collect()).collect();
    let mut result_silver = 0;
    let mut result_gold = 0;

    for machine in machines {
        let light_target = get_target(&machine);
        let joltage_target = get_joltage_target(&machine);
        let combinations = calculate_button_combinations(&get_buttons(&machine), light_target.len());

        let target_key = light_target.iter().map(|&x| if x == 0 { '0' } else { '1' }).collect::<String>();
        let min_buttons_to_light_target = combinations
            .get(&target_key)
            .map(|combos| combos.iter().map(|combo| combo.total_presses).min().unwrap_or(usize::MAX))
            .unwrap_or(usize::MAX);

        let min_buttons_to_joltage_target = minimum_buttons_to_joltage_target(
            &joltage_target,
            &combinations,
            &mut HashMap::new()
        );

        result_silver += min_buttons_to_light_target;
        result_gold += min_buttons_to_joltage_target;
    }

    (Box::new(result_silver), Box::new(result_gold))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2025, 10, silver, gold)
        .with_both(both)
}

fn get_target(machine: &[&str]) -> Vec<usize> {
    machine[0]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect()
}

fn get_buttons(machine: &[&str]) -> Vec<Vec<usize>> {
    machine[1..machine.len() - 1]
        .iter()
        .map(|&b| get_button(b))
        .collect()
}

fn get_button(button: &str) -> Vec<usize> {
    button
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_joltage_target(machine: &[&str]) -> Vec<i64> {
    machine
        .last()
        .unwrap()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Clone)]
struct ButtonCombination {
    total_presses: usize,
    effect: Vec<usize>,
}

fn calculate_button_combinations(
    buttons: &[Vec<usize>],
    target_length: usize,
) -> HashMap<String, Vec<ButtonCombination>> {
    let mut combinations: HashMap<String, Vec<ButtonCombination>> = HashMap::new();
    calculate_button_combinations_recursive(
        buttons,
        target_length,
        0,
        vec![0; target_length],
        0,
        &mut combinations,
    );
    combinations
}

fn calculate_button_combinations_recursive(
    buttons: &[Vec<usize>],
    target_length: usize,
    start_index: usize,
    current_effect: Vec<usize>,
    total_presses: usize,
    combinations: &mut HashMap<String, Vec<ButtonCombination>>,
) {
    let key: String = current_effect
        .iter()
        .map(|&x| if x % 2 == 0 { '0' } else { '1' })
        .collect();

    combinations
        .entry(key)
        .or_insert_with(Vec::new)
        .push(ButtonCombination {
            total_presses,
            effect: current_effect.clone(),
        });

    for i in start_index..buttons.len() {
        let button = &buttons[i];
        let mut new_effect = current_effect.clone();
        for &index in button {
            new_effect[index] += 1;
        }
        calculate_button_combinations_recursive(
            buttons,
            target_length,
            i + 1,
            new_effect,
            total_presses + 1,
            combinations,
        );
    }
}

fn get_target_parity_key(target: &[i64]) -> String {
    target
        .iter()
        .map(|&x| if x % 2 == 0 { '0' } else { '1' })
        .collect()
}

fn minimum_buttons_to_joltage_target(
    target: &[i64],
    combinations: &HashMap<String, Vec<ButtonCombination>>,
    target_cache: &mut HashMap<String, usize>,
) -> usize {
    let target_cache_key = target
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    if let Some(&cached) = target_cache.get(&target_cache_key) {
        return cached;
    }

    if target.iter().all(|&t| t == 0) {
        return 0;
    }

    if target.iter().any(|&t| t < 0) {
        return usize::MAX;
    }

    let target_key = get_target_parity_key(target);
    if !combinations.contains_key(&target_key) {
        return usize::MAX;
    }

    let mut current_min = usize::MAX;
    for combo in &combinations[&target_key] {
        let new_target: Vec<i64> = target
            .iter()
            .enumerate()
            .map(|(i, &t)| (t - combo.effect[i] as i64) / 2)
            .collect();

        let res = minimum_buttons_to_joltage_target(&new_target, combinations, target_cache);
        if res != usize::MAX {
            current_min = current_min.min(res * 2 + combo.total_presses);
        }
    }

    target_cache.insert(target_cache_key, current_min);
    current_min
}

fn calculate_minimum_button_presses_for_target(
    target: &[usize],
    buttons: &[Vec<usize>],
) -> usize {
    calculate_minimum_button_presses_recursive(
        target,
        buttons,
        0,
        vec![0; target.len()],
        0,
        buttons.len(),
    )
}

fn calculate_minimum_button_presses_recursive(
    target: &[usize],
    buttons: &[Vec<usize>],
    start_index: usize,
    current_effect: Vec<usize>,
    total_presses: usize,
    mut min_buttons: usize,
) -> usize {
    if total_presses >= min_buttons {
        return min_buttons;
    }

    if current_effect == target {
        return total_presses;
    }

    for i in start_index..buttons.len() {
        let button = &buttons[i];
        let mut new_effect = current_effect.clone();
        for &index in button {
            new_effect[index] = if new_effect[index] == 1 { 0 } else { 1 };
        }

        let result = calculate_minimum_button_presses_recursive(
            target,
            buttons,
            i + 1,
            new_effect,
            total_presses + 1,
            min_buttons,
        );
        min_buttons = min_buttons.min(result);
    }

    min_buttons
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

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(TEST_INPUT).to_string(), "7");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2025, 10);
        assert_eq!(silver(&input).to_string(), "486");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold(TEST_INPUT).to_string(), "33");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2025, 10);
        assert_eq!(gold(&input).to_string(), "17820");
    }
}
