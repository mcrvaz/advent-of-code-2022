use std::{
    collections::HashSet,
    fs::read_to_string,
    ops::{Add, Mul},
};

pub fn solve() {
    let result = internal_solve("src/days/day9/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> usize {
    let content = read_to_string(path).expect("Fail to read file.");
    let commands = parse(&content);
    let mut tail_positions: Vec<Point> = vec![Point::ZERO];
    let mut head_pos: Point = Point::ZERO;
    for c in commands {
        head_pos = c.exec(&head_pos, &mut tail_positions);
    }
    let unique_positions: HashSet<Point> = HashSet::from_iter(tail_positions.iter().cloned());
    unique_positions.iter().count()
}

fn parse(content: &str) -> Vec<Command> {
    content.lines().map(|x| Command::from_str(x)).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        const MAX_DIST: i32 = 1;
        (self.x - other.x) <= MAX_DIST && (self.y - other.y) <= MAX_DIST
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
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
        fn get_new_tail_position(head: &Point, current_tail: &Point) -> Point {
            const MAX_DIST: i32 = 1;
            if head.is_adjacent(current_tail) {
                return *current_tail;
            }
            let mut new_tail = *current_tail;
            let x_dist = head.x - current_tail.x;
            let y_dist = head.y - current_tail.y;
            if x_dist.abs() > MAX_DIST {
                new_tail.x += x_dist
            }

            if y_dist.abs() > MAX_DIST {
                new_tail.y += x_dist
            }

            new_tail
        }

        let mut new_head_pos = *head_pos;
        for _ in 0..self.count {
            new_head_pos = new_head_pos + self.dir;
            let previous_tail_pos = tail_positions.last().unwrap();
            let new_tail_pos = get_new_tail_position(&new_head_pos, previous_tail_pos);
            tail_positions.push(new_tail_pos);
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
    fn result() {
        const PATH: &str = "src/days/day9/input.txt";
        const EXPECTED: usize = 0;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
