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

    // height = 99, width = 99
    let height = input.len();
    let width = input[0].len();
    // println!("height: {}, width: {}", height, width);
    let mut seen = vec![vec![false; width]; height];

    // top and bottom
    for x in 1..width - 1 {
        let mut y = 0;
        while input[y][x] < input[y + 1][x] {
            seen[y + 1][x] = true;
            y += 1;
        }

        let mut y = height - 1;
        while input[y][x] < input[y - 1][x] {
            seen[y - 1][x] = true;
            y -= 1;
        }
    }

    // left and right
    for y in 1..height - 1 {
        let mut x = 0;
        while input[y][x] < input[y][x + 1] {
            seen[y][x + 1] = true;
            x += 1;
        }

        let mut x = width - 1;
        while input[y][x] < input[y][x - 1] {
            println!("y: {y} x: {x}");
            seen[y][x - 1] = true;
            x -= 1;
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
