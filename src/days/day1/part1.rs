use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day1/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    get_max_calories(&content)
}

fn get_max_calories(content: &str) -> i32 {
    let mut current = 0;
    let mut max = 0;
    for line in content.lines() {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap();
    }
    if current > max {
        max = current;
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day1/test-input.txt";
        const EXPECTED: i32 = 24000;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day1/input.txt";
        const EXPECTED: i32 = 71934;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
