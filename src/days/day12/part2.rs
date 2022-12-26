use std::{
    cell::RefCell,
    fs::read_to_string,
    hash::{Hash, Hasher},
    rc::Rc,
};

use pathfinding::prelude::dijkstra;

pub fn solve() {
    let result = internal_solve("src/days/day12/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> u32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let graph = parse(&content);
    let start_nodes = graph.get_start_nodes();
    let mut smallest_path = u32::MAX;
    for start in start_nodes {
        let s = &*start.borrow();
        let path: Option<(Vec<Node>, u32)> = dijkstra(s, |x| x.get_successors(), |x| x.is_end);
        if let Some(p) = path {
            if p.1 < smallest_path {
                smallest_path = p.1;
            }
        }
    }
    smallest_path
}

fn parse(content: &str) -> Graph {
    let mut grid: Vec<Vec<Rc<RefCell<Node>>>> = Vec::new();
    let mut id = 0;
    for (i, line) in content.lines().enumerate() {
        grid.insert(i, Vec::new());
        for (j, c) in line.chars().enumerate() {
            let node = Node::from_char(id, c);
            grid[i].insert(j, Rc::new(RefCell::new(node)));
            id += 1;
        }
    }

    let height = grid.len();
    let width = grid[0].len();
    for i in 0..height {
        for j in 0..width {
            let mut current = grid.get(i).and_then(|x| x.get(j)).unwrap().borrow_mut();
            if is_valid_position(i as i32 - 1, j as i32, width as i32, height as i32) {
                current.try_add_sibling(grid[i - 1][j].clone());
            }
            if is_valid_position(i as i32 + 1, j as i32, width as i32, height as i32) {
                current.try_add_sibling(grid[i + 1][j].clone());
            }
            if is_valid_position(i as i32, j as i32 - 1, width as i32, height as i32) {
                current.try_add_sibling(grid[i][j - 1].clone());
            }
            if is_valid_position(i as i32, j as i32 + 1, width as i32, height as i32) {
                current.try_add_sibling(grid[i][j + 1].clone());
            }
        }
    }

    Graph { nodes: grid }
}

fn is_valid_position(i: i32, j: i32, width: i32, height: i32) -> bool {
    (i >= 0 && i < height) && (j >= 0 && j < width)
}

fn get_elevation(c: char) -> u32 {
    match c {
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        c => c as u32,
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Rc<RefCell<Node>>>>,
}

impl Graph {
    fn get_start_nodes(&self) -> Vec<Rc<RefCell<Node>>> {
        let height = self.nodes.len();
        let width = self.nodes[0].len();
        let mut result = Vec::new();
        for i in 0..height {
            for j in 0..width {
                if self.nodes[i][j].borrow().is_start {
                    result.push(self.nodes[i][j].clone());
                }
            }
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    id: usize,
    is_start: bool,
    is_end: bool,
    weight: u32,
    siblings: Vec<Rc<RefCell<Node>>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Node {
    fn from_char(id: usize, c: char) -> Self {
        Node {
            id,
            is_start: c == 'S' || c == 'a',
            is_end: c == 'E',
            weight: get_elevation(c),
            siblings: Vec::new(),
        }
    }

    fn try_add_sibling(&mut self, node: Rc<RefCell<Node>>) {
        let sibling = node.borrow();
        if sibling.weight <= self.weight + 1 {
            self.siblings.push(node.clone());
        }
    }

    fn get_successors(&self) -> Vec<(Self, u32)> {
        self.siblings
            .iter()
            .map(|x| (x.borrow().clone(), 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day12/test-input.txt";
        const EXPECTED: u32 = 29;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day12/input.txt";
        const EXPECTED: u32 = 345;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
