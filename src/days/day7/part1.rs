use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() {
    let result = internal_solve("src/days/day7/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    // const SIZE_LIMIT: i32 = 100000;
    let content = read_to_string(path).expect("Fail to read file.");
    let mut fs = FileSystem::new();
    for line in content.lines() {
        if let Some(param) = parse_cd(&line) {
            fs.execute_cd(param);
            continue;
        } else if parse_ls(&line) {
            continue;
        } else if let Some(dir) = parse_dir(&line) {
            fs.register_dir(dir);
        } else if let Some(file) = parse_file(&line) {
            fs.register_file(file);
        } else {
            panic!("Invalid command!");
        }
    }
    -1
}

fn parse_cd(line: &str) -> Option<&str> {
    let cd_rx = Regex::new(r"$ cd (.*)").unwrap();
    let captures = cd_rx.captures(line).unwrap();
    captures.get(1).and_then(|x| Some(x.as_str()))
}

fn parse_ls(line: &str) -> bool {
    line == "$ ls"
}

fn parse_dir(line: &str) -> Option<Directory> {
    let dir_rx = Regex::new(r"dir (\w)").unwrap();
    let captures = dir_rx.captures(line).unwrap();
    captures
        .get(1)
        .and_then(|x| Some(x.as_str()))
        .and_then(|x| Some(Directory::from_name(x)))
}

fn parse_file(line: &str) -> Option<File> {
    let file_rx = Regex::new(r"({\d}*) ({.*})").unwrap();
    let captures = file_rx.captures(line).unwrap();
    let name = captures.get(1).and_then(|x| Some(x.as_str()));
    let size = captures
        .get(2)
        .and_then(|x| Some(x.as_str()))
        .and_then(|x| Some(x.parse().unwrap()));

    if let (Some(name), Some(size)) = (name, size) {
        Some(File {
            name: name.to_string(),
            size: size,
        })
    } else {
        None
    }
}

struct FileSystem {
    dirs: HashMap<String, Directory>,
    current_dir: String,
}

impl FileSystem {
    fn new() -> Self {
        let name = String::from("/");
        let root = Directory::from_name(&name);
        let dirs = HashMap::from_iter(vec![(root.name.to_string(), root)]);
        FileSystem {
            dirs: dirs,
            current_dir: name,
        }
    }

    fn execute_cd(&mut self, param: &str) {
        self.current_dir = match param {
            "/" => self.dirs["/"].name.to_string(),
            ".." => self.get_parent_dir().name.to_string(),
            dir_name => self.enter_dir(&dir_name).name.to_string(),
            _ => panic!("Invalid param for cd."),
        };
    }

    fn enter_dir(&mut self, dir_name: &str) -> &Directory {
        self.current_dir = self.dirs[dir_name].name.to_string();
        &self.dirs[&self.current_dir]
    }

    fn get_parent_dir(&mut self) -> &Directory {
        // "/"
        // "/a"
        // "/a/b"

        if self.current_dir == "/" {
            return self.dirs.get(&self.current_dir).unwrap();
        }

        // TODO this is probably wrong
        let split = self.current_dir.split("/").next().unwrap();
        return self.dirs.get(split).unwrap();
    }

    fn register_dir(&mut self, dir: Directory) {
        let mut current_dir = self.current_dir.clone();
        current_dir.push_str("/");
        current_dir.push_str(&dir.name);
        self.dirs.insert(current_dir, dir);
    }

    fn register_file(&mut self, file: File) {
        let dir = self.dirs.get_mut(&self.current_dir).unwrap();
        dir.add_file(file);
    }
}

struct File {
    pub name: String,
    pub size: i32,
}

struct Directory {
    pub name: String,
    pub files: Vec<File>,
}

impl Directory {
    fn from_name(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
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

    // #[test]
    // fn result() {
    //     const PATH: &str = "src/days/day5/input.txt";
    //     const EXPECTED: &str = "QMBMJDFTD";
    //     let result = internal_solve(PATH);
    //     assert_eq!(result, EXPECTED);
    // }
}
