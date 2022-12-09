use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| {
            let mut line = line.split(' ');
            (
                line.next().unwrap().parse::<Direction>().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut seen = HashSet::new();

    for (direction, amount) in input.into_iter() {
        for _ in 0..amount {
            match direction {
                Direction::Up => {
                    head_pos.1 += 1;
                }
                Direction::Down => {
                    head_pos.1 -= 1;
                }
                Direction::Left => {
                    head_pos.0 += 1;
                }
                Direction::Right => {
                    head_pos.0 -= 1;
                }
            }

            follow_head(&head_pos, &mut tail_pos);

            seen.insert(tail_pos);
            println!("{:?} {:?}", head_pos, tail_pos);
        }
    }

    println!("{:?}", seen.len());
}

fn follow_head(head_pos: &(i32, i32), tail_pos: &mut (i32, i32)) {
    let delta_x = (head_pos.0 - tail_pos.0).abs();
    let delta_y = (head_pos.1 - tail_pos.1).abs();

    if delta_x + delta_y == 3 {
        if delta_x == 1 {
            tail_pos.0 = head_pos.0;
            tail_pos.1 = (head_pos.1 + tail_pos.1) / 2;
        } else {
            tail_pos.1 = head_pos.1;
            tail_pos.0 = (head_pos.0 + tail_pos.0) / 2;
        }
        return;
    }

    // head is too up
    if head_pos.0 - tail_pos.0 > 1 {
        tail_pos.0 = head_pos.0 - 1;
    }

    // head is too down
    if tail_pos.0 - head_pos.0 > 1 {
        tail_pos.0 = head_pos.0 + 1;
    }

    // head is too left
    if head_pos.1 - tail_pos.1 > 1 {
        tail_pos.1 = head_pos.1 - 1;
    }

    // head is too left
    if tail_pos.1 - head_pos.1 > 1 {
        tail_pos.1 = head_pos.1 + 1;
    }
}
