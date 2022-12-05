use itertools::izip;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split("\n\n").collect();
    if let &[stacks, procedure] = &input[..] {
        let mut stacks = parse_stacks(stacks);
        let procedure = parse_procedure(procedure);

        for (n, from, to) in procedure {
            for _ in 0..n {
                let item = stacks[from].pop().unwrap();
                stacks[to].push(item);
            }
        }

        let result: String = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect();

        println!("{:?}", result);
    }
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let stacks: Vec<_> = stacks
        .split('\n')
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect();

    let stacks: Vec<_> = izip!(
        &stacks[0], &stacks[1], &stacks[2], &stacks[3], &stacks[4], &stacks[5], &stacks[6],
        &stacks[7]
    )
    .collect();

    let stacks = stacks
        .iter()
        .map(|(c0, c1, c2, c3, c4, c5, c6, c7)| {
            vec![c0, c1, c2, c3, c4, c5, c6, c7]
                .iter()
                .filter_map(|&&&c| if c != ' ' { Some(c) } else { None })
                .collect()
        })
        .map(|stack: Vec<_>| stack.into_iter().rev().collect())
        .collect();

    // println!("{:?}", &stacks);
    stacks
}

fn parse_procedure(procedure: &str) -> Vec<(usize, usize, usize)> {
    let procedure = procedure
        .split('\n')
        .map(|line| {
            let line: Vec<_> = line.split(' ').collect();
            (
                line[1].parse::<usize>().unwrap(),
                line[3].parse::<usize>().unwrap() - 1,
                line[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    // println!("{:?}", &procedure[0..5]);
    procedure
}
