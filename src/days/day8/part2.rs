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
    get_result(tree.root.clone())
}

fn get_result(root_ref: Rc<RefCell<TreeNode>>) -> i32 {
    const TOTAL_SIZE: i32 = 70000000;
    const TARGET_SIZE: i32 = 30000000;
    let root = root_ref.borrow();
    let available_space = TOTAL_SIZE - root.size;
    let space_needed = TARGET_SIZE - available_space;

    let mut sizes = Vec::new();
    get_sizes(root_ref.clone(), &mut sizes);
    fn get_sizes(root_ref: Rc<RefCell<TreeNode>>, vec: &mut Vec<i32>) {
        let node = root_ref.borrow();
        vec.push(node.size);
        for c in root_ref.borrow().children.iter() {
            get_sizes(c.clone(), vec);
        }
    }

    sizes
        .iter()
        .filter(|x| **x >= space_needed)
        .min()
        .map(|x| *x)
        .unwrap()
}

fn parse(content: &str) -> Tree {
    let tree = Tree::new();
    let mut tree_pointer = tree.root.clone();
    for line in content.lines() {
        if let Some(param) = parse_cd(&line) {
            match param {
                "/" => {
                    continue;
                }
                ".." => {
                    move_to_upper_dir(&mut tree_pointer);
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
            tree_pointer.borrow_mut().increase_size(file.size)
        } else {
            panic!("Invalid command!");
        }
    }

    while tree_pointer.borrow().parent.is_some() {
        move_to_upper_dir(&mut tree_pointer);
    }
    tree
}

fn move_to_upper_dir(tree_pointer: &mut Rc<RefCell<TreeNode>>) {
    let current_size = tree_pointer.borrow().size.clone();
    let parent = tree_pointer.borrow().parent.clone().unwrap();
    parent.borrow_mut().increase_size(current_size);
    *tree_pointer = parent;
}

fn parse_cd(line: &str) -> Option<&str> {
    lazy_static! {
        static ref CD_RX: Regex = Regex::new(r"\$ cd (\S+)").unwrap();
    }

    if let Some(c) = CD_RX.captures(line) {
        return c.get(1).and_then(|x| Some(x.as_str()));
    }
    None
}

fn parse_ls(line: &str) -> bool {
    line == "$ ls"
}

fn parse_dir(line: &str) -> Option<Directory> {
    lazy_static! {
        static ref DIR_RX: Regex = Regex::new(r"dir (\S+)").unwrap();
    }

    if let Some(c) = DIR_RX.captures(line) {
        return c.get(1).and_then(|x| Some(x.as_str())).and_then(|x| {
            Some(Directory {
                name: x.to_string(),
            })
        });
    }
    None
}

fn parse_file(line: &str) -> Option<File> {
    lazy_static! {
        static ref FILE_RX: Regex = Regex::new(r"(\d+) (\S+)").unwrap();
    }

    if let Some(c) = FILE_RX.captures(line) {
        let size = c
            .get(1)
            .and_then(|x| Some(x.as_str()))
            .and_then(|x| Some(x.parse().unwrap()));
        let name = c.get(2).and_then(|x| Some(x.as_str()));
        if let (Some(size), _) = (size, name) {
            return Some(File { size });
        }
    }
    None
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
        Tree { root }
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

    fn increase_size(&mut self, incr: i32) {
        self.size += incr;
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
        const EXPECTED: i32 = 24933642;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        println!("my version");
        const PATH: &str = "src/days/day7/input.txt";
        const EXPECTED: i32 = 3636703;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
