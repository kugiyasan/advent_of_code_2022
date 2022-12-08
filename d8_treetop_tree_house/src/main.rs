use std::fs;

fn is_inner_tree_visible(
    input: &Vec<Vec<u32>>,
    (x, y): (usize, usize),
    (inner_x, inner_y): (usize, usize),
) -> bool {
    input[y][x] < input[inner_y][inner_x]
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Vec<_>> = buf
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let height = input.len();
    let width = input[0].len();
    let mut res = 0;

    for x in 2..width - 2 {
        if input[0][x] < input[1][x] {
            res += 1;
        }
        if input[height - 1][x] < input[height - 2][x] {
            res += 1;
        }
    }
    println!("{}", res);

    for y in 2..height - 2 {
        if input[0][y] < input[1][y] {
            res += 1;
        }
        if input[y][width - 1] < input[y][width - 2] {
            res += 1;
        }
    }
    println!("{}", res);

    // Corners
    if input[0][1] < input[1][1] || input[1][0] < input[1][1] {
        res += 1;
    }
    if is_inner_tree_visible(&input, (1, width - 1), (1, width - 2))
        || is_inner_tree_visible(&input, (0, width - 2), (1, width - 2))
    {
        res += 1;
    }
    if is_inner_tree_visible(&input, (height - 1, 1), (height - 2, 1))
        || is_inner_tree_visible(&input, (height - 2, 0), (height - 2, 1))
    {
        res += 1;
    }
    if input[height - 1][width - 2] < input[height - 2][width - 2]
        || input[height - 2][width - 1] < input[height - 2][width - 2]
    {
        res += 1;
    }
    println!("{}", res);

    let res = res + height * 2 + (width - 2) * 2;
    println!("{:?}", &input[0..5]);
    println!("{}", res);
}
