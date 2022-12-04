use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day4/input.txt");
    println!("Result: {}", result);
}

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(range: &str) -> Self {
        let mut pair = range.split('-');
        let first_start: i32 = pair.next().unwrap().parse().unwrap();
        let first_end: i32 = pair.next().unwrap().parse().unwrap();
        Range::new(first_start, first_end)
    }

    fn new(start: i32, end: i32) -> Self {
        Range {
            start: start,
            end: end,
        }
    }

    fn contains(self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut count = 0;
    for line in content.lines() {
        let mut pairs = line.split(',');
        let first_range = Range::from_str(pairs.next().unwrap());
        let second_range = Range::from_str(pairs.next().unwrap());
        if first_range.contains(second_range) || second_range.contains(first_range) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day4/test-input.txt";
        const EXPECTED: i32 = 2;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day4/input.txt";
        const EXPECTED: i32 = 576;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
