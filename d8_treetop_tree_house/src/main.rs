use std::fs;

fn main() {
    // let path = "input";
    let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Vec<_>> = buf
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // println!("input:\n{}", buf);
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<u32>>) {
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
    println!("{}", res);
}

fn part2(input: &Vec<Vec<u32>>) {
    let height = input.len();
    let width = input[0].len();
    let max = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .map(|x| scenic_score(input, y, x))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("max: {}", max);
}

fn scenic_score(input: &Vec<Vec<u32>>, tree_y: usize, tree_x: usize) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut count_above = 0;
    let mut max_tree_height = 0;
    for y in (0..tree_y).rev() {
        if input[y][tree_x] > max_tree_height {
            count_above += 1;
            max_tree_height = input[y][tree_x];
        }
    }

    let mut count_below = 0;
    let mut max_tree_height = 0;
    for y in tree_y + 1..height {
        if input[y][tree_x] > max_tree_height {
            count_below += 1;
            max_tree_height = input[y][tree_x];
        }
    }

    let mut count_left = 0;
    let mut max_tree_height = 0;
    for x in (0..tree_x).rev() {
        if input[tree_y][x] > max_tree_height {
            count_left += 1;
            max_tree_height = input[tree_y][x];
        }
    }

    let mut count_right = 0;
    let mut max_tree_height = 0;
    for x in tree_x + 1..width {
        if input[tree_y][x] > max_tree_height {
            count_right += 1;
            max_tree_height = input[tree_y][x];
        }
    }

    println!(
        "({}, {}) {} {} {} {}",
        tree_y, tree_x, count_above, count_below, count_left, count_right
    );
    count_above * count_below * count_left * count_right
    // (count_above + 1) * (count_below + 1) * (count_left + 1) * (count_right + 1)
}
