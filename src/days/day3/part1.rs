use std::{collections::HashSet, fs::read_to_string};

pub fn solve() {
    let result = internal_solve("src/days/day3/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut sum = 0;
    for line in content.lines() {
        let half_len = line.len() / 2;
        let first_compart: HashSet<char> = HashSet::from_iter(line[..half_len].chars());
        let second_compart: HashSet<char> = HashSet::from_iter(line[half_len..].chars());
        let intersection = first_compart.intersection(&second_compart);
        sum += intersection.map(|x| get_priority(*x)).sum::<i32>();
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
        const EXPECTED: i32 = 157;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day3/input.txt";
        const EXPECTED: i32 = 8105;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
