use std::{fs::read_to_string, str::FromStr};

pub fn solve() {
    let result = internal_solve("src/days/day4/input.txt");
    println!("Result: {}", result);
}

struct Range(std::ops::Range<i32>);

#[derive(Debug)]
struct FromStrErr;

impl FromStr for Range {
    type Err = FromStrErr;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        Ok(range
            .split_once('-')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .map(|(start, end)| Range(std::ops::Range { start, end }))
            .unwrap())
    }
}

trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
}

impl Contains<Range> for Range {
    fn contains(&self, other: &Range) -> bool {
        self.0.start <= other.0.start && self.0.end >= other.0.end
    }
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut count = 0;
    for line in content.lines() {
        let (r1, r2) = line
            .split_once(',')
            .map(|(x, y)| (Range::from_str(x).unwrap(), Range::from_str(y).unwrap()))
            .unwrap();
        if r1.contains(&r2) || r2.contains(&r1) {
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
