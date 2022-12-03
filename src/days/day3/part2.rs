use std::{collections::HashSet, fs::read_to_string};

pub fn solve() {
    let result = internal_solve("src/days/day3/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    const GROUP_SIZE: usize = 3;
    let content = read_to_string(path).expect("Fail to read file.");
    let mut sum = 0;
    let mut group: Vec<HashSet<char>> = Vec::with_capacity(GROUP_SIZE);
    for line in content.lines() {
        group.push(HashSet::from_iter(line.chars()));

        if group.len() == GROUP_SIZE {
            let first = group.iter().next().unwrap().clone();
            let intersection = group
                .iter()
                .fold(first, |acc, next| acc.intersection(next).copied().collect());
            sum += intersection.iter().map(|x| get_priority(*x)).sum::<i32>();
            group.clear();
        }
    }
    sum
}

fn get_priority(c: char) -> i32 {
    const LOWERCASE_COUNT: i32 = 'z' as i32 - 'a' as i32 + 1;
    if is_uppercase(c) {
        let diff = c as i32 - 'A' as i32;
        diff + LOWERCASE_COUNT + 1
    } else {
        let diff = c as i32 - 'a' as i32;
        diff + 1
    }
}

fn is_uppercase(c: char) -> bool {
    c < 'a'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day3/test-input.txt";
        const EXPECTED: i32 = 70;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day3/input.txt";
        const EXPECTED: i32 = 2363;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
