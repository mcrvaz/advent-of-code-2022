use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() {
    let result = internal_solve("src/days/day7/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    const SIZE_LIMIT: i32 = 100000;
    let content = read_to_string(path).expect("Fail to read file.");
    let mut fs = FileSystem::new();
    for line in content.lines() {
        if let Some(param) = parse_cd(&line) {
            fs.enter_dir(param);
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
    println!("{}", fs.current_dir);

    fs.dirs
        .values()
        .map(|x| x.get_size())
        .filter(|x| *x < SIZE_LIMIT)
        .sum()
}

fn parse_cd(line: &str) -> Option<&str> {
    let cd_rx = Regex::new(r"\$ cd (\S+)").unwrap();
    let captures = cd_rx.captures(line);
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
    let dir_rx = Regex::new(r"dir (\w)").unwrap();
    let captures = dir_rx.captures(line);
    if let Some(c) = captures {
        c.get(1)
            .and_then(|x| Some(x.as_str()))
            .and_then(|x| Some(Directory::from_name(x)))
    } else {
        None
    }
}

fn parse_file(line: &str) -> Option<File> {
    let file_rx = Regex::new(r"(\d+) (\S+)").unwrap();
    let captures = file_rx.captures(line);
    if let Some(c) = captures {
        let size = c
            .get(1)
            .and_then(|x| Some(x.as_str()))
            .and_then(|x| Some(x.parse().unwrap()));
        let name = c.get(2).and_then(|x| Some(x.as_str()));
        if let (Some(size), Some(name)) = (size, name) {
            Some(File {
                size: size,
                name: name.to_string(),
            })
        } else {
            None
        }
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
        let mut fs = FileSystem {
            dirs: HashMap::new(),
            current_dir: String::from(""),
        };
        fs.register_dir(Directory::from_name(""));
        fs
    }

    fn get_dir_name(&self, name: &str) -> String {
        if name == ".." {
            let parent_dir = self.get_parent_dir().name.to_string();
            if parent_dir == "/" {
                return parent_dir;
            } else {
                let mut final_dir = "/".to_string();
                final_dir.push_str(&parent_dir);
                return final_dir;
            }
        }

        let parent_dir = &self.current_dir;
        if name == "/" || parent_dir == "/" {
            let mut dir = parent_dir.to_string();
            dir.push_str(name);
            return dir;
        } else {
            let mut dir = parent_dir.to_string();
            dir.push_str("/");
            dir.push_str(name);
            return dir;
        }
    }

    fn enter_dir(&mut self, dir_name: &str) -> &Directory {
        let name = self.get_dir_name(dir_name);
        println!("{}", name);
        let d = self.dirs.get(&name).unwrap();
        self.current_dir = name;
        d
    }

    fn get_parent_dir(&self) -> &Directory {
        if self.current_dir == "/" {
            return self.dirs.get(&self.current_dir).unwrap();
        }
        let (parent, _) = self.current_dir.rsplit_once("/").unwrap();
        if parent.is_empty() {
            return self.dirs.get("/").unwrap();
        }
        return self.dirs.get(parent).unwrap();
    }

    fn register_dir(&mut self, dir: Directory) {
        let mut current_dir = self.current_dir.clone();
        if current_dir != "/" {
            current_dir.push_str("/");
        }
        current_dir.push_str(&dir.name);
        self.dirs.insert(current_dir, dir);
    }

    fn register_file(&mut self, file: File) {
        let name = self.current_dir.to_string();
        let dir = self.dirs.get_mut(&name).unwrap();
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

    fn get_size(&self) -> i32 {
        self.files.iter().map(|x| x.size).sum()
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
