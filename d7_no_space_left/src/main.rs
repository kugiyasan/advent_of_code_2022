use std::{collections::HashMap, fs, iter, str::FromStr};

#[derive(Debug, Clone)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Clone)]
enum Line {
    Command(Command),
    File(String, usize),
    Folder(String),
}

impl FromStr for Line {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split(' ');
        let first = line.next().unwrap();
        let second = line.next().unwrap();
        let third = line.next();
        match first {
            "$" => match second {
                "cd" => Ok(Line::Command(Command::Cd(third.unwrap().to_string()))),
                "ls" => Ok(Line::Command(Command::Ls)),
                _ => Err(()),
            },
            "dir" => Ok(Line::Folder(second.to_string())),
            _ => Ok(Line::File(second.to_string(), first.parse().unwrap())),
        }
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Line> = buf
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut directory: Vec<String> = Vec::new();
    let mut folders = HashMap::new();

    for line in input {
        match line {
            Line::Command(command) => match command {
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        directory.clear();
                        directory.push(path)
                    }
                    ".." => {
                        directory.pop().unwrap();
                    }
                    _ => directory.push(path),
                },
                Command::Ls => (),
            },
            Line::File(name, size) => {
                let mut directory = directory.iter().chain(iter::once(&name));
                let mut path = directory.next().unwrap().clone();
                for dir in directory {
                    folders.insert(path.clone(), folders.get(&path).unwrap_or(&0) + size);
                    path += dir;
                    path += "/";
                }
            }
            Line::Folder(_) => (),
        }
    }

    let sum: usize = folders.values().filter(|&&size| size < 100000).sum();

    let missing_space = 30_000_000 - (70_000_000 - folders.get("/").unwrap());
    let sizes: Vec<_> = folders
        .values()
        .filter(|&&size| size >= missing_space)
        .collect();

    let size = sizes.iter().min().unwrap();

    println!("part 1: {}", sum);
    println!("missing_space {}", missing_space);
    println!("part 2: {}", size);
}
