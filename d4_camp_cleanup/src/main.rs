use std::{
    fs,
    ops::{Range, RangeInclusive},
};

fn check_if_complete_overlap(r1: Range<i32>, r2: Range<i32>) -> bool {
    let r1_in_r2 = r1.start <= r2.start && r1.end >= r2.end;
    let r2_in_r1 = r1.start >= r2.start && r1.end <= r2.end;
    r1_in_r2 || r2_in_r1
}

fn check_if_partial_overlap(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> bool {
    r1.start() <= r2.end() && r1.end() >= r2.start()
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| {
            let numbers: Vec<_> = line
                .split(',')
                .take(2)
                .flat_map(|range| {
                    range
                        .split('-')
                        .take(2)
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect();
            numbers
        })
        .collect();

        print_answer(&input, true);
        print_answer(&input, false);
}

fn print_answer(input: &[Vec<i32>], part1: bool) {
    let result: Vec<_> = input
        .iter()
        .map(|numbers| match numbers[..] {
            [n1, n2, n3, n4] => {
                if part1 {
                    i32::from(check_if_complete_overlap(n1..n2, n3..n4))
                } else {
                    i32::from(check_if_partial_overlap(n1..=n2, n3..=n4))
                }
            }
            _ => {
                panic!("No 4 numbers");
            }
        })
        .collect();

    let sum: i32 = result.iter().sum();

    println!("{:?}", &result[0..5]);
    println!("{}", sum);
}
