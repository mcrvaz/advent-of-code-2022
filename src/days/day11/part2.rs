use std::{collections::HashMap, fs::read_to_string, str::Lines};

use lazy_static::lazy_static;
use regex::Regex;

const ROUNDS: i32 = 10000;

pub fn solve() {
    let result = internal_solve("src/days/day11/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> u128 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut iter = content.lines();
    let mut monkeys = HashMap::new();
    while let Some(monkey) = parse_monkey(&mut iter) {
        monkeys.insert(monkey.id, monkey);
    }

    let factor = monkeys.values().map(|x| x.test.divisible_by).product();
    let mut keys: Vec<i32> = monkeys.keys().copied().collect();
    keys.sort();
    for _ in 1..=ROUNDS {
        for monkey_id in &keys {
            let monkey = monkeys.get_mut(&monkey_id).unwrap();
            if monkey.items.is_empty() {
                continue;
            }
            monkey.inspect(factor);
            let throws = monkey.test();
            for throw in throws {
                let target_monkey = monkeys.get_mut(&throw.monkey_id).unwrap();
                target_monkey.receive(throw.value);
            }
        }
    }

    let mut values: Vec<u128> = monkeys.values().map(|x| x.inspection_count).collect();
    values.sort_by(|a, b| b.cmp(a));
    values[0] * values[1]
}

fn parse_monkey(lines_iter: &mut Lines) -> Option<Monkey> {
    if let Some(line) = lines_iter.next() {
        let id = parse_monkey_id(line);
        let starting_items = parse_starting_items(lines_iter.next().unwrap());
        let op = parse_operation(lines_iter.next().unwrap());
        let test = parse_test(lines_iter);
        _ = lines_iter.next();
        return Some(Monkey {
            id,
            items: starting_items,
            op,
            test,
            inspection_count: 0,
        });
    };
    None
}

fn parse_monkey_id(line: &str) -> i32 {
    lazy_static! {
        static ref RX: Regex = Regex::new(r"Monkey (\S+):").unwrap();
    }
    if let Some(c) = RX.captures(line) {
        return c
            .get(1)
            .and_then(|x| Some(x.as_str()))
            .map(|x| x.parse::<i32>().unwrap())
            .unwrap();
    }
    panic!("Couldn't parse monkey id!");
}

fn parse_starting_items(line: &str) -> Vec<u128> {
    let (_, starting_items_str) = line.split_once(":").unwrap();
    starting_items_str
        .split(",")
        .map(|x| x.trim())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    fn parse_value(v: &str) -> OperationValue {
        match v {
            "old" => OperationValue::Old,
            v => OperationValue::Value(v.parse().unwrap()),
        }
    }
    fn parse_operator(v: &str) -> Operator {
        match v {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Invalid operator!"),
        }
    }

    lazy_static! {
        static ref RX: Regex = Regex::new(r"Operation: new = (\S+) (\S+) (\S+)").unwrap();
    }

    if let Some(c) = RX.captures(line) {
        let lhs = c
            .get(1)
            .and_then(|x| Some(x.as_str()))
            .map(|x| parse_value(x))
            .unwrap();
        let op = c
            .get(2)
            .and_then(|x| Some(x.as_str()))
            .map(|x| parse_operator(x))
            .unwrap();
        let rhs = c
            .get(3)
            .and_then(|x| Some(x.as_str()))
            .map(|x| parse_value(x))
            .unwrap();
        return Operation { lhs, op, rhs };
    }
    panic!("Couldn't parse operation!");
}

fn parse_test(lines_iter: &mut Lines) -> MonkeyTest {
    fn last_to_i64(line: &str, split_v: &str) -> u128 {
        line.split_once(split_v)
            .iter()
            .last()
            .map(|x| x.1.parse().unwrap())
            .unwrap()
    }
    let div: u128 = last_to_i64(lines_iter.next().unwrap(), "Test: divisible by ");
    let m1: i32 = last_to_i64(lines_iter.next().unwrap(), "If true: throw to monkey ") as i32;
    let m2: i32 = last_to_i64(lines_iter.next().unwrap(), "If false: throw to monkey ") as i32;
    MonkeyTest {
        divisible_by: div,
        true_monkey: m1,
        false_monkey: m2,
    }
}

struct Monkey {
    id: i32,
    items: Vec<u128>,
    op: Operation,
    test: MonkeyTest,
    inspection_count: u128,
}

impl Monkey {
    fn inspect(&mut self, factor: u128) {
        for v in self.items.iter_mut() {
            *v = self.op.apply(*v) % factor;
            self.inspection_count += 1;
        }
    }

    fn test(&mut self) -> Vec<MonkeyThrow> {
        let mut throws = Vec::new();
        for v in self.items.iter() {
            if v % self.test.divisible_by == 0 {
                throws.push(MonkeyThrow {
                    value: *v,
                    monkey_id: self.test.true_monkey,
                });
            } else {
                throws.push(MonkeyThrow {
                    value: *v,
                    monkey_id: self.test.false_monkey,
                });
            }
        }
        self.items.clear();
        throws
    }

    fn receive(&mut self, item: u128) {
        self.items.push(item);
    }
}

struct Operation {
    lhs: OperationValue,
    op: Operator,
    rhs: OperationValue,
}

impl Operation {
    fn apply(&self, value: u128) -> u128 {
        let lhs = match self.lhs {
            OperationValue::Old => value,
            OperationValue::Value(v) => v,
        };
        let rhs = match self.rhs {
            OperationValue::Old => value,
            OperationValue::Value(v) => v,
        };

        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

enum OperationValue {
    Old,
    Value(u128),
}
enum Operator {
    Add,
    Mul,
}

struct MonkeyTest {
    divisible_by: u128,
    true_monkey: i32,
    false_monkey: i32,
}

struct MonkeyThrow {
    value: u128,
    monkey_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day11/test-input.txt";
        const EXPECTED: u128 = 2713310158;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day11/input.txt";
        const EXPECTED: u128 = 14081365540;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
