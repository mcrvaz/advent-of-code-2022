use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

pub fn solve() {
    let result = internal_solve("src/days/day7/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let tree = parse(&content);
    let result = get_result(&tree);
    result
}

fn get_result(tree: &Tree) -> i32 {
    let result = get_size_sum(tree.root.clone());
    result
}

fn get_size_sum(node_ref: Rc<RefCell<TreeNode>>) -> i32 {
    const SIZE_LIMIT: i32 = 100000;

    let node = node_ref.borrow();
    let size = node.get_full_node_size();
    let mut size_sum = 0;
    if size <= SIZE_LIMIT {
        size_sum += size;
    }

    for c in node.children.iter() {
        let size = get_size_sum(c.clone());
        if size <= SIZE_LIMIT {
            size_sum += size;
        }
    }
    size_sum
}

fn parse(content: &str) -> Tree {
    let tree = Tree::new();
    {
        let mut tree_pointer = tree.root.clone();
        for line in content.lines() {
            if let Some(param) = parse_cd(&line) {
                match param {
                    "/" => {
                        continue;
                    }
                    ".." => {
                        let parent = tree_pointer.borrow().parent.clone();
                        tree_pointer = parent.unwrap();
                    }
                    p => {
                        let target = tree_pointer.borrow().find_child(p).unwrap();
                        tree_pointer = target;
                    }
                }
            } else if parse_ls(&line) {
                continue;
            } else if let Some(dir) = parse_dir(&line) {
                tree_pointer
                    .borrow_mut()
                    .add_child(tree_pointer.clone(), &dir.name);
            } else if let Some(file) = parse_file(&line) {
                tree_pointer.borrow_mut().increase_value(file.size)
            } else {
                panic!("Invalid command!");
            }
        }
    }
    tree
}

fn parse_cd(line: &str) -> Option<&str> {
    lazy_static! {
        static ref CD_RX: Regex = Regex::new(r"\$ cd (\S+)").unwrap();
    }

    let captures = CD_RX.captures(line);
    if let Some(c) = captures {
        c.get(1).and_then(|x| Some(x.as_str()))
    } else {
        None
    }
}

fn parse_ls(line: &str) -> bool {
    line == "$ ls"
}

fn parse_dir(line: &str) -> Option<Directory> {
    lazy_static! {
        static ref DIR_RX: Regex = Regex::new(r"dir (\S+)").unwrap();
    }

    let captures = DIR_RX.captures(line);
    if let Some(c) = captures {
        c.get(1).and_then(|x| Some(x.as_str())).and_then(|x| {
            Some(Directory {
                name: x.to_string(),
            })
        })
    } else {
        None
    }
}

fn parse_file(line: &str) -> Option<File> {
    lazy_static! {
        static ref FILE_RX: Regex = Regex::new(r"(\d+) (\S+)").unwrap();
    }

    let captures = FILE_RX.captures(line);
    if let Some(c) = captures {
        let size = c
            .get(1)
            .and_then(|x| Some(x.as_str()))
            .and_then(|x| Some(x.parse().unwrap()));
        let name = c.get(2).and_then(|x| Some(x.as_str()));
        if let (Some(size), _) = (size, name) {
            Some(File { size: size })
        } else {
            None
        }
    } else {
        None
    }
}

struct Tree {
    root: Rc<RefCell<TreeNode>>,
}

struct TreeNode {
    size: i32,
    name: String,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl Tree {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(TreeNode {
            size: 0,
            parent: None,
            name: String::from("/"),
            children: Vec::new(),
        }));
        Tree { root: root }
    }
}

impl TreeNode {
    fn add_child(&mut self, parent: Rc<RefCell<TreeNode>>, name: &str) {
        let node = Rc::new(RefCell::new(TreeNode {
            size: 0,
            name: name.to_string(),
            parent: Some(parent),
            children: Vec::new(),
        }));
        self.children.push(node);
    }

    fn find_child(&self, name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let target = self.children.iter().find(|x| x.borrow().name == name);
        target.map(|x| x.clone())
    }

    fn increase_value(&mut self, incr: i32) {
        self.size += incr;
    }

    fn get_full_node_size(&self) -> i32 {
        let mut size_sum = self.size;
        for c in self.children.iter() {
            size_sum += c.borrow().get_full_node_size();
        }
        size_sum
    }
}

struct File {
    size: i32,
}

struct Directory {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day7/test-input.txt";
        const EXPECTED: i32 = 95437;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day7/input.txt";
        // 29178 too low
        const EXPECTED: i32 = -1;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
