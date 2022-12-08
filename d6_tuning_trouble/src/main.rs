use std::{collections::HashSet, fs};

fn main() {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    // let buf = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();

    // let unique_chars = 4;
    let unique_chars = 14;

    let index = buf
        .chars()
        .collect::<Vec<_>>()
        .windows(unique_chars)
        .position(|arr| {
            let hash_set = HashSet::<_>::from_iter(arr.iter());
            hash_set.len() == unique_chars
        })
        .unwrap();

    println!("{:?}", index + unique_chars);
}
