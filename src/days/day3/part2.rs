use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day3/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        // const PATH: &str = "src/days/day2/test-input.txt";
        // const EXPECTED: i32 = 15;
        // let result = internal_solve(PATH);
        // assert_eq!(result, EXPECTED);
        assert!(false);
    }
}
