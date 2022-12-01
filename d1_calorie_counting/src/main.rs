use std::{fs::File, io::Read};

fn main() {
    let path = "input";
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    // let input: Vec<Vec<i32>> = buf
    //     .trim()
    //     .split("\n\n")
    //     .map(|s| s.split('\n').map(|line| line.parse().unwrap()).collect())
    //     .collect();

    let input: i32 = buf
        .trim()
        .split("\n\n")
        .map(|s| s.split('\n').map(|line| line.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("{:?}", input);
}
