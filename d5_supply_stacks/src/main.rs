use std::fs;

fn main() {
    // let path = "input";
    // let path = "example";
    let path = "aoc_2022_day05_large_input.txt";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split("\n\n").collect();
    if let &[stacks, procedure] = &input[..] {
        let mut stacks = parse_stacks(stacks);
        let procedure = parse_procedure(procedure);
        let mut swap_buffer = vec![0u8; 100_000_000];

        for (n, from, to) in procedure {
            // https://github.com/sprunq/AdventOfCode/blob/aab6973d2950a5edc398bb2cdfdc6a15dc088c79/AoC2022/src/aoc5/mod.rs#L80
            let len_to_swap = stacks[from].len();
            let swap_buffer = &mut swap_buffer[0..n];
            swap_buffer.copy_from_slice(&stacks[from][len_to_swap - n..len_to_swap]);
            stacks[from].truncate(len_to_swap - n);
            stacks[to].extend_from_slice(swap_buffer);
        }

        let result: String = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap() as char)
            .collect();

        println!("{}", result);
        // println!("{:?}", stacks);
        // println!("{:?}", procedure);
    }
}

fn parse_stacks(string: &str) -> Vec<Vec<u8>> {
    let mut stacks = vec![Vec::<u8>::new(); 9];
    for line in string.lines() {
        for (i, c) in line.bytes().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }
    }
    stacks
        .into_iter()
        .map(|v| v.into_iter().rev().collect())
        .collect()
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
