// use aoc_utils::submit;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";

    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<((i32, i32), (i32, i32))> = buf.trim().split('\n').map(parse_line).collect();

    let upper_bound = if path == "input" { 4000000 } else { 20 };
    let (min_x, max_x, min_y, max_y) = get_bounding_box(&input, upper_bound);
    let min_y = 44000;
    let max_y = 1000000;

    (min_y..=max_y)
        .collect::<Vec<_>>()
        .into_par_iter()
        .chunks(100)
        .for_each(|range| {
            println!("y: {}", range[0]);

            for y in range {
                let mut x = min_x;
                while x <= max_x {
                    x += guess_cell(&input, x, y);
                }
            }
        });
}

fn guess_cell(input: &Vec<((i32, i32), (i32, i32))>, x: i32, y: i32) -> i32 {
    for (sensor, beacon) in input.iter() {
        if sensor == &(x, y) {
            return 1;
        }

        if beacon == &(x, y) {
            return 1;
        }

        let dist1 = get_manhattan_distance(sensor, beacon);
        let dist2 = get_manhattan_distance(sensor, &(x, y));
        if dist1 >= dist2 {
            // println!("{:?} {:?} {} {}", (x, y), sensor, dist1, dist2);
            let dx = sensor.0 - x;
            if dx > 0 {
                return dx;
            }
            return 1;
        }
    }

    let tuning_frequency = x * 4000000 + y;
    println!("x: {} y: {} tuning_frequency: {}", x, y, tuning_frequency);
    return 1;
}

fn get_bounding_box(
    input: &Vec<((i32, i32), (i32, i32))>,
    upper_bound: i32,
) -> (i32, i32, i32, i32) {
    let min_x = input
        .iter()
        .map(|((x1, _y1), (x2, _y2))| x1.min(x2))
        .min()
        .unwrap();
    let max_x = input
        .iter()
        .map(|((x1, _y1), (x2, _y2))| x1.max(x2))
        .max()
        .unwrap();
    let min_y = input
        .iter()
        .map(|((_x1, y1), (_x2, y2))| y1.min(y2))
        .min()
        .unwrap();
    let max_y = input
        .iter()
        .map(|((_x1, y1), (_x2, y2))| y1.max(y2))
        .max()
        .unwrap();

    let min_x = min_x.max(&0);
    let max_x = max_x.min(&upper_bound);
    let min_y = min_y.max(&0);
    let max_y = max_y.min(&upper_bound);

    println!(
        "min_x: {} max_x: {} min_y: {} max_y: {}",
        min_x, max_x, min_y, max_y
    );

    (*min_x, *max_x, *min_y, *max_y)
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let coords = line[10..]
        .split(": closest beacon is at ")
        .map(parse_coord)
        .collect::<Vec<_>>();

    (coords[0], coords[1])
}

fn parse_coord(coord: &str) -> (i32, i32) {
    let coord = coord
        .split(", ")
        .map(|n| n[2..].parse().unwrap())
        .collect::<Vec<_>>();

    (coord[0], coord[1])
}

fn get_manhattan_distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx.abs() + dy.abs()
}
