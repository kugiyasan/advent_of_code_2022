use std::{collections::HashSet, fs};

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<(&str, &str)> = buf
        .trim()
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    let result: Vec<i32> = input
        .iter()
        .map(|(compartment1, compartment2)| {
            let set = HashSet::<_>::from_iter(compartment1.chars());
            let letter = compartment2.chars().find(|c| set.contains(c)).unwrap();
            if letter.is_lowercase() {
                (letter as u8 - 'a' as u8 + 1) as i32
            } else {
                (letter as u8 - 'A' as u8 + 27) as i32
            }
        })
        .collect();

    let sum: i32 = result.iter().sum();

    // println!("{:?}", input);
    // println!("{:?}", input[0]);
    // println!("{:?}", &result[0..5]);
    println!("{}", sum);
}
