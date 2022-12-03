use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day2/input.txt");
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
    fn test_case() {
        assert!(false);
    }
}
