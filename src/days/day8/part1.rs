use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day8/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> u32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let trees = parse_trees(&content);
    get_visible_tree_count(&trees)
}

fn parse_trees(content: &str) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::with_capacity(content.lines().count());
    for (i, line) in content.lines().enumerate() {
        trees.insert(i, Vec::with_capacity(line.len()));
        for (j, c) in line.chars().enumerate() {
            trees[i].insert(j, c.to_digit(10).unwrap());
        }
    }
    trees
}

fn get_visible_tree_count(trees: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            if is_visible(row, col, &trees) {
                count += 1;
            }
        }
    }
    count
}

fn is_visible(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    is_visible_from_top(row, col, trees)
        || is_visible_from_bot(row, col, trees)
        || is_visible_from_left(row, col, trees)
        || is_visible_from_right(row, col, trees)
}

fn is_visible_from_top(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    let value = trees[row][col];
    for i in 0..row {
        let current = trees[i][col];
        if current >= value {
            return false;
        }
    }
    true
}

fn is_visible_from_bot(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    let value = trees[row][col];
    let row_count = trees.len();
    for i in (row + 1)..row_count {
        let current = trees[i][col];
        if current >= value {
            return false;
        }
    }
    true
}

fn is_visible_from_left(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    let value = trees[row][col];
    for i in 0..col {
        let current = trees[row][i];
        if current >= value {
            return false;
        }
    }
    true
}

fn is_visible_from_right(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    let value = trees[row][col];
    let col_count = trees[row].len();
    for i in (col + 1)..col_count {
        let current = trees[row][i];
        if current >= value {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day8/test-input.txt";
        const EXPECTED: u32 = 21;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day8/input.txt";
        const EXPECTED: u32 = 1700;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
