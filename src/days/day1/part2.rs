use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day1/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    const GROUP_COUNT: usize = 3;
    let content = read_to_string(path).expect("Fail to read file.");
    get_max_calories(&content, GROUP_COUNT)
}

fn get_max_calories(content: &str, group_count: usize) -> i32 {
    let mut result = Vec::with_capacity(group_count);
    let mut current = 0;

    for line in content.lines() {
        if line.is_empty() {
            try_replace_smaller_value(current, &mut result);
            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap();
    }
    try_replace_smaller_value(current, &mut result);

    result.iter().sum()
}

fn try_replace_smaller_value(element: i32, vec: &mut Vec<i32>) {
    if vec.len() < vec.capacity() {
        vec.push(element);
        return;
    }

    vec.sort();
    for i in 0..vec.len() {
        if element > vec[i] {
            vec[i] = element;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day1/test-input.txt";
        const EXPECTED: i32 = 45000;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day1/input.txt";
        const EXPECTED: i32 = 211447;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
