use std::{collections::HashSet, fs};

fn get_letter_priority(letter: char) -> i32 {
    if letter.is_lowercase() {
        (letter as u8 - b'a' + 1) as i32
    } else {
        (letter as u8 - b'A' + 27) as i32
    }
}

fn part1(buf: &str) {
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
            get_letter_priority(letter)
        })
        .collect();

    let sum: i32 = result.iter().sum();
    println!("{}", sum);
}

fn part2(buf: &str) {
    let input: Vec<&str> = buf.trim().split('\n').collect();
    let result: Vec<_> = input
        .chunks(3)
        .map(|elves| {
            if let &[elf1, elf2, elf3] = elves {
                let set1 = HashSet::<_>::from_iter(elf1.chars());
                let set2 = HashSet::<_>::from_iter(elf2.chars());
                let set3 = HashSet::<_>::from_iter(elf3.chars());

                let intersection1: HashSet<_> = set1
                    .intersection(&set2)
                    .map(|c| c.to_owned())
                    .collect::<HashSet<_>>();
                let letters: Vec<_> = intersection1.intersection(&set3).collect();

                if let Some(first) = letters.first() {
                    **first
                } else {
                    panic!("There's more than one letter");
                }
            } else {
                panic!("Not 3 lines");
            }
        })
        .collect();

    let sum: i32 = result
        .iter()
        .map(|&letter| get_letter_priority(letter))
        .sum();

    println!("{}", sum);
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();
    part1(&buf);
    part2(&buf);
}
