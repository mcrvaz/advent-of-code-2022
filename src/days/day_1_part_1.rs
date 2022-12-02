use std::fs::read_to_string;

pub fn solve() {
    const PATH: &str = "src/days/input.txt";
    let content = read_to_string(PATH).expect("Fail to read file.");
    let max = get_max_calories(&content);
    println!("Result: {}", max);
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
