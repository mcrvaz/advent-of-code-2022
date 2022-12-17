use std::{collections::HashSet, fs::read_to_string, ops::Add};

pub fn solve() {
    let result = internal_solve("src/days/day9/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> usize {
    let content = read_to_string(path).expect("Fail to read file.");
    let commands = parse(&content);
    let tail_positions = compute_tail_positions(&commands);
    let unique_positions: HashSet<Point> = HashSet::from_iter(tail_positions.iter().cloned());
    unique_positions.iter().count()
}

fn parse(content: &str) -> Vec<Command> {
    content.lines().map(|x| Command::from_str(x)).collect()
}

fn compute_tail_positions(commands: &Vec<Command>) -> Vec<Point> {
    let mut tail_positions: Vec<Point> = vec![Point::ZERO];
    let mut head_pos: Point = Point::ZERO;
    for c in commands {
        head_pos = c.exec(&head_pos, &mut tail_positions);
    }
    tail_positions
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };
    const R: Point = Point { x: 1, y: 0 };
    const L: Point = Point { x: -1, y: 0 };
    const U: Point = Point { x: 0, y: 1 };
    const D: Point = Point { x: 0, y: -1 };

    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from_char(c: char) -> Self {
        match c {
            'R' => Point::R,
            'L' => Point::L,
            'U' => Point::U,
            'D' => Point::D,
            _ => panic!("Invalid direction!"),
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        const MAX_DIST: u32 = 1;
        self.x.abs_diff(other.x) <= MAX_DIST && self.y.abs_diff(other.y) <= MAX_DIST
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Command {
    dir: Point,
    count: i32,
}

impl Command {
    fn from_str(str: &str) -> Self {
        let (dir, count) = str.split_once(" ").unwrap();
        Command {
            dir: Point::from_char(dir.chars().next().unwrap()),
            count: count.parse().unwrap(),
        }
    }

    fn exec(&self, head_pos: &Point, tail_positions: &mut Vec<Point>) -> Point {
        fn get_new_tail_position(head: &Point, current_tail: &Point) -> Option<Point> {
            if head.is_adjacent(current_tail) {
                return None;
            }

            let y_diff_sig = (head.y - current_tail.y).signum();
            let x_diff_sig = (head.x - current_tail.x).signum();
            let mut x = 0;
            let mut y = 0;
            if head.x == current_tail.x {
                y = y_diff_sig;
            } else if head.y == current_tail.y {
                x = x_diff_sig;
            } else {
                y = y_diff_sig;
                x = x_diff_sig;
            }
            let new_tail = *current_tail + Point::new(x, y);
            Some(new_tail)
        }

        let mut new_head_pos = *head_pos;
        for _ in 0..self.count {
            new_head_pos = new_head_pos + self.dir;
            let previous_tail_pos = tail_positions.last().unwrap();
            if let Some(p) = get_new_tail_position(&new_head_pos, previous_tail_pos) {
                tail_positions.push(p);
            }
        }
        new_head_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day9/test-input.txt";
        const EXPECTED: usize = 13;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample_path() {
        const PATH: &str = "src/days/day9/test-input.txt";
        let content = read_to_string(PATH).expect("Fail to read file.");
        let commands = parse(&content);
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(3, 4),
            Point::new(2, 4),
            Point::new(3, 3),
            Point::new(4, 3),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
        ];
        let result = compute_tail_positions(&commands);
        assert_eq!(result, expected);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day9/input.txt";
        const EXPECTED: usize = 6087;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
