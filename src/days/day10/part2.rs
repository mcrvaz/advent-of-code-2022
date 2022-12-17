use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day9/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> u32 {
    let content = read_to_string(path).expect("Fail to read file.");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day9/test-input.txt";
        const EXPECTED: u32 = 13140;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day9/input.txt";
        const EXPECTED: u32 = u32::MAX;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
