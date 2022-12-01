use std::{fs::File, io::Read};

fn main() {
    let path = "input";
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    let mut input: Vec<i32> = buf
        .trim()
        .split("\n\n")
        .map(|s| s.split('\n').map(|line| line.parse::<i32>().unwrap()).sum())
        .collect();

    println!("{}", input.iter().max().unwrap());

    // part 2

    input.sort();
    input.reverse();

    let sum: i32 = input.iter().take(3).sum();

    println!("{sum}");
}
