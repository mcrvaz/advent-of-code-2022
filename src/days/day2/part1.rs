use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day2/input.txt");
    println!("Result: {}", result);
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(input: char) -> Shape {
        match input {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid shape input."),
        }
    }

    fn get_value(self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut score = 0;
    for line in content.lines() {
        let opponent = Shape::from_char(line.chars().nth(0).unwrap());
        let player = Shape::from_char(line.chars().nth(2).unwrap());
        score += get_round_result(player, opponent);
    }
    score
}

fn get_round_result(player_in: Shape, opponent_in: Shape) -> i32 {
    let shape_value = player_in.get_value();
    let match_value = get_match_value(player_in, opponent_in);
    shape_value + match_value
}

fn get_match_value(player_in: Shape, opponent_in: Shape) -> i32 {
    const VICTORY: i32 = 6;
    const DRAW: i32 = 3;
    const DEFEAT: i32 = 0;

    if player_in == opponent_in {
        return DRAW;
    }

    let won = match player_in {
        Shape::Rock => opponent_in == Shape::Scissors,
        Shape::Paper => opponent_in == Shape::Rock,
        Shape::Scissors => opponent_in == Shape::Paper,
    };

    if won {
        VICTORY
    } else {
        DEFEAT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        const PATH: &str = "src/days/day2/test-input.txt";
        const EXPECTED: i32 = 15;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
