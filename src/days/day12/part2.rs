use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day12/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day12/test-input.txt";
        const EXPECTED: i32 = -1;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day12/input.txt";
        const EXPECTED: i32 = -1;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
