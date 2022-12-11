use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day8/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> u32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let trees = parse_trees(&content);
    get_highest_score(&trees)
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

fn get_highest_score(trees: &Vec<Vec<u32>>) -> u32 {
    let mut highest_score = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let score = get_scenic_score(row, col, &trees);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    highest_score
}

fn get_scenic_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    get_top_score(row, col, trees)
        * get_bot_score(row, col, trees)
        * get_left_score(row, col, trees)
        * get_right_score(row, col, trees)
}

fn get_top_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let value = trees[row][col];
    let mut score = 0;
    for i in (0..row).rev() {
        let current = trees[i][col];
        score += 1;
        if current >= value {
            return score;
        }
    }
    score
}

fn get_bot_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let value = trees[row][col];
    let row_count = trees.len();
    let mut score = 0;
    for i in (row + 1)..row_count {
        let current = trees[i][col];
        score += 1;
        if current >= value {
            return score;
        }
    }
    score
}

fn get_left_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let value = trees[row][col];
    let mut score = 0;
    for i in (0..col).rev() {
        let current = trees[row][i];
        score += 1;
        if current >= value {
            return score;
        }
    }
    score
}

fn get_right_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let value = trees[row][col];
    let col_count = trees[row].len();
    let mut score = 0;

    for i in (col + 1)..col_count {
        let current = trees[row][i];
        score += 1;
        if current >= value {
            return score;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day8/test-input.txt";
        const EXPECTED: u32 = 8;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day8/input.txt";
        const EXPECTED: u32 = 470596;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
