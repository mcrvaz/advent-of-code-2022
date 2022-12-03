use std::fs::read_to_string;

pub fn solve() {
    let result = internal_solve("src/days/day2/input.txt");
    println!("Result: {}", result);
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum MatchResult {
    Victory = 6,
    Draw = 3,
    Defeat = 0,
}

impl MatchResult
{
    fn from_char(input: char) -> MatchResult {
        match input {
            'X' => MatchResult::Defeat,
            'Y' => MatchResult::Draw,
            'Z' => MatchResult::Victory,
            _ => panic!("Invalid match result input."),
        }
    }
}

impl Shape {
    fn from_char(input: char) -> Shape {
        match input {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Invalid shape input."),
        }
    }

    fn get_winning_shape(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn get_losing_shape(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut score = 0;
    for line in content.lines() {
        let opponent = Shape::from_char(line.chars().nth(0).unwrap());
        let target_result = MatchResult::from_char(line.chars().nth(2).unwrap());
        let player = get_shape_to_match_result(opponent, target_result);
        score += get_round_result(player, opponent);
    }
    score
}

fn get_round_result(player_in: Shape, opponent_in: Shape) -> i32 {
    let shape_value = player_in as i32;
    let match_value = get_match_value(player_in, opponent_in);
    shape_value + match_value
}

fn get_shape_to_match_result(opponent_in: Shape, target_result: MatchResult) -> Shape {
    match target_result {
        MatchResult::Victory => opponent_in.get_winning_shape(),
        MatchResult::Draw => opponent_in,
        MatchResult::Defeat => opponent_in.get_losing_shape(),
    }
}

fn get_match_value(player_in: Shape, opponent_in: Shape) -> i32 {
    if player_in == opponent_in {
        return MatchResult::Draw as i32
    }

    let won = match player_in {
        Shape::Rock => opponent_in == Shape::Scissors,
        Shape::Paper => opponent_in == Shape::Rock,
        Shape::Scissors => opponent_in == Shape::Paper,
    };

    if won {
        MatchResult::Victory as i32
    } else {
        MatchResult::Defeat as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        const PATH: &str = "src/days/day2/test-input.txt";
        const EXPECTED: i32 = 12;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
