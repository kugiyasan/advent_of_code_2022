use std::{fs, ops::Range};

fn check_if_complete_overlap(r1: Range<i32>, r2: Range<i32>) -> bool {
    let r1_in_r2 = r1.start <= r2.start && r1.end >= r2.end;
    let r2_in_r1 = r1.start >= r2.start && r1.end <= r2.end;
    r1_in_r2 || r2_in_r1
}

fn main() {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| {
            let numbers: Vec<_> = line
                .split(',')
                .take(2)
                .map(|range| {
                    range
                        .split('-')
                        .take(2)
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .flatten()
                .collect();
            numbers
        })
        .collect();

    let result: Vec<_> = input
        .iter()
        .map(|numbers| match numbers[..] {
            [n1, n2, n3, n4] => {
                if check_if_complete_overlap(n1..n2, n3..n4) {
                    1
                } else {
                    0
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
