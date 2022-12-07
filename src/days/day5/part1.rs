use regex::Regex;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt,
    fs::read_to_string,
    str::Lines,
};

pub fn solve() {
    let result = internal_solve("src/days/day5/input.txt");
    println!("Result: {}", result);
}

struct Instruction {
    pub orig: usize,
    pub dst: usize,
    pub count: usize,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.count, self.orig, self.dst)
    }
}

fn internal_solve(path: &str) -> String {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut line_iter = content.lines();
    let mut stacks = parse_stacks(&mut line_iter);
    let instructions: Vec<Instruction> = line_iter.map(|x| parse_instruction(x)).collect();
    for inst in instructions {
        execute_instruction(&mut stacks, inst)
    }

    stacks
        .values()
        .map(|x| x.clone().pop_back().unwrap())
        .collect::<String>()
}

fn parse_stacks(line_iter: &mut Lines) -> BTreeMap<usize, VecDeque<char>> {
    const SPACING: usize = 1;
    const ELEMENT_SIZE: usize = 3;
    const RADIX: u32 = 10;

    let mut result: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();

    loop {
        if let Some(line) = line_iter.next() {
            let mut chars = line.chars();
            let mut i = 0;
            while let Some(c) = chars.next() {
                i += 1;
                if c.is_whitespace() {
                    continue;
                }
                if c.is_digit(RADIX) {
                    line_iter.next();
                    return result;
                }

                if c == '[' {
                    let value = chars.next().unwrap();
                    i += 1;
                    let stack_number = ((i as usize) / (SPACING + ELEMENT_SIZE)) + 1;
                    if result.contains_key(&stack_number) {
                        result
                            .get_mut(&stack_number)
                            .and_then(|x| Some(x.push_front(value)));
                    } else {
                        let mut vec = VecDeque::new();
                        vec.push_front(value);
                        result.insert(stack_number, vec);
                    }
                    _ = chars.next();
                    i += 1;
                }
            }
        }
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let rx = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = rx.captures(line).unwrap();
    Instruction {
        dst: captures.get(3).unwrap().as_str().parse().unwrap(),
        orig: captures.get(2).unwrap().as_str().parse().unwrap(),
        count: captures.get(1).unwrap().as_str().parse().unwrap(),
    }
}

fn execute_instruction(stacks: &mut BTreeMap<usize, VecDeque<char>>, inst: Instruction) {
    for _ in 0..inst.count {
        let value = stacks.get_mut(&inst.orig).unwrap().pop_back().unwrap();
        stacks.get_mut(&inst.dst).unwrap().push_back(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day5/test-input.txt";
        const EXPECTED: &str = "CMZ";
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day5/input.txt";
        const EXPECTED: &str = "QMBMJDFTD";
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
