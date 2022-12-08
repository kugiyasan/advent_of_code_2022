use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Vec<_>> = buf
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // height = 99, width = 99
    let height = input.len();
    let width = input[0].len();
    // println!("height: {}, width: {}", height, width);
    let mut seen = vec![vec![false; width]; height];

    // top and bottom
    for x in 1..width - 1 {
        let mut max_tree_height = input[0][x];
        for y in 1..height - 1 {
            if input[y][x] > max_tree_height {
                seen[y][x] = true;
                max_tree_height = input[y][x];
            }
        }

        let mut max_tree_height = input[height - 1][x];
        for y in (1..height - 1).rev() {
            if input[y][x] > max_tree_height {
                seen[y][x] = true;
                max_tree_height = input[y][x];
            }
        }
    }

    // left and right
    for y in 1..height - 1 {
        let mut max_tree_height = input[y][0];
        for x in 1..width - 1 {
            if input[y][x] > max_tree_height {
                seen[y][x] = true;
                max_tree_height = input[y][x];
            }
        }

        let mut max_tree_height = input[y][width - 1];
        for x in (1..width - 1).rev() {
            // println!("y: {y} x: {x}");
            if input[y][x] > max_tree_height {
                seen[y][x] = true;
                max_tree_height = input[y][x];
            }
        }
    }

    println!(
        "seen:\n{}",
        seen.iter()
            .map(|line| line
                .iter()
                .map(|&b| if b { '1' } else { '0' })
                .collect::<String>()
                + "\n")
            .collect::<String>()
    );
    let res: usize = seen
        .into_iter()
        .map(|line| line.iter().filter(|&&b| b).count())
        .sum();
    println!("{}", res);
    let res = res + height * 2 + (width - 2) * 2;
    println!("input:\n{}", buf);
    println!("{}", res);
}
