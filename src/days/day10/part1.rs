use std::{collections::HashMap, fs::read_to_string};

pub fn solve() {
    let result = internal_solve("src/days/day10/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut cycle = 1;
    let mut register = 1;
    let mut target_values: HashMap<i32, i32> =
        HashMap::from_iter([(20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)]);

    for line in content.lines() {
        if let Some(x) = parse_addx(line) {
            cycle += 1;
            evaluate_target_value(&mut target_values, cycle, register);
            cycle += 1;
            register += x;
        } else {
            cycle += 1;
        }

        evaluate_target_value(&mut target_values, cycle, register);
    }
    target_values.iter().fold(0, |acc, v| acc + (v.0 * v.1))
}

fn evaluate_target_value(target_values: &mut HashMap<i32, i32>, cycle: i32, register: i32) {
    if target_values.contains_key(&cycle) {
        target_values.insert(cycle, register);
    }
}

fn parse_addx(line: &str) -> Option<i32> {
    line.split_once(" ").map(|x| x.1.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day10/test-input.txt";
        const EXPECTED: i32 = 13140;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day10/input.txt";
        const EXPECTED: i32 = 14920;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
