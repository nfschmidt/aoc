use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Read},
};

#[derive(Debug)]
struct Directory {
    directories: HashMap<String, Box<Directory>>,
    files: HashMap<String, u32>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            directories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn mkdir(&mut self, path: &str, name: &str) {
        let parent = self.get_dir(path);

        // if the dir exists, do nothing
        if let Some(_) = parent.directories.get(name) {
            return;
        }

        parent
            .directories
            .insert(name.to_string(), Box::new(Directory::new()));
    }

    fn mkfile(&mut self, path: &str, name: &str, size: u32) {
        let parent = self.get_dir(path);

        // if the file exists, do nothing
        if let Some(_) = parent.directories.get(name) {
            return;
        }

        parent.files.insert(name.to_string(), size);
    }

    fn get_dir(&mut self, path: &str) -> &mut Directory {
        if path == "" || path == "/" {
            return self;
        }

        let dirs = path.split("/").collect::<Vec<_>>();
        let mut current = self;
        for d in dirs.iter().skip(1) {
            current = current.directories.get_mut(&d.to_string()).unwrap();
        }

        current
    }

    fn print(&self, depth: usize) {
        if depth == 0 {
            println!("- / (dir)");
        }

        for (name, dir) in self.directories.iter() {
            println!("{}- {} (dir)", "  ".repeat(depth + 1), name);
            dir.print(depth + 2);
        }
        for (name, size) in self.files.iter() {
            println!("{}- {} (file, size={})", "  ".repeat(depth + 1), name, size);
        }
    }

    fn size(&self) -> u32 {
        let dirs_size: u32 = self.directories.values().map(|d| d.size()).sum();
        let files_size: u32 = self.files.values().sum();

        return dirs_size + files_size;
    }
}

enum ParsingStatus {
    ReadingCommand,
    ReadingList,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut fs = Directory::new();
    let mut lines = input.lines();
    let mut next = lines.next();
    let mut current_dir = VecDeque::new();
    let mut parsing_status = ParsingStatus::ReadingCommand;

    while let Some(line) = next {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        match parsing_status {
            ParsingStatus::ReadingCommand => {
                match *fields.get(1).unwrap() {
                    "cd" => match *fields.get(2).unwrap() {
                        "/" => current_dir.clear(),
                        ".." => _ = current_dir.pop_back(),
                        name => {
                            fs.mkdir(
                                &current_dir
                                    .iter()
                                    .fold("".to_string(), |acc, p| format!("{}/{}", acc, p)),
                                name,
                            );
                            current_dir.push_back(name);
                        }
                    },
                    "ls" => parsing_status = ParsingStatus::ReadingList,
                    _ => panic!("unexpected line: {}", line),
                };
            }
            ParsingStatus::ReadingList => match *fields.get(0).unwrap() {
                "dir" => fs.mkdir(
                    &current_dir
                        .iter()
                        .fold("".to_string(), |acc, p| format!("{}/{}", acc, p)),
                    *fields.get(1).unwrap(),
                ),
                "$" => {
                    parsing_status = ParsingStatus::ReadingCommand;
                    continue;
                }
                size_str => fs.mkfile(
                    &current_dir
                        .iter()
                        .fold("".to_string(), |acc, p| format!("{}/{}", acc, p)),
                    *fields.get(1).unwrap(),
                    size_str.parse().unwrap(),
                ),
            },
        }

        next = lines.next();
    }

    let part1_result: u32 = get_all_sizes(&fs)
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum();
    println!("{}", part1_result);

    let free_space = 70_000_000 - fs.size();
    let needed_space = 30_000_000 - free_space;

    let mut candidates = get_all_sizes(&fs)
        .into_iter()
        .filter(|&s| s >= needed_space)
        .collect::<Vec<_>>();

    candidates.sort();

    let part2_result = candidates.get(0).unwrap();

    println!("{}", part2_result);
}

fn get_all_sizes(dir: &Directory) -> Vec<u32> {
    let mut result = Vec::new();
    result.push(dir.size());

    for dir in dir.directories.values() {
        let mut sizes = get_all_sizes(dir);
        result.append(&mut sizes);
    }

    result
}
