use std::fs::read_to_string;

pub fn solve() {
    const PATH: &str = "src/days/test-input.txt";
    const GROUP_COUNT: usize = 3;
    let content = read_to_string(PATH).expect("Fail to read file.");
    let max = get_max_calories(&content, GROUP_COUNT);
    println!("Result: {}", max);
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
