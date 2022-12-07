use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day6/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> usize {
    const DIFF_CHAR_COUNT: usize = 4;
    let content = read_to_string(path).expect("Fail to read file.");
    let mut values: Vec<char> = Vec::with_capacity(DIFF_CHAR_COUNT);
    for (i, v) in content.chars().enumerate() {
        if values.len() == DIFF_CHAR_COUNT {
            return i;
        }

        let idx = values.iter().position(|x| *x == v);
        if let Some(idx) = idx {
            values.drain(0..=idx);
        }
        values.push(v);
    }
    panic!("No result found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        const PATH: &str = "src/days/day6/test-input-1.txt";
        const EXPECTED: usize = 7;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample_2() {
        const PATH: &str = "src/days/day6/test-input-2.txt";
        const EXPECTED: usize = 5;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample_3() {
        const PATH: &str = "src/days/day6/test-input-3.txt";
        const EXPECTED: usize = 6;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample_4() {
        const PATH: &str = "src/days/day6/test-input-4.txt";
        const EXPECTED: usize = 10;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample_5() {
        const PATH: &str = "src/days/day6/test-input-5.txt";
        const EXPECTED: usize = 11;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day6/input.txt";
        const EXPECTED: usize = 1155;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
