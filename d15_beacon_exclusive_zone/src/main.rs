// use aoc_utils::submit;
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";

    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<((i32, i32), (i32, i32))> = buf.trim().split('\n').map(parse_line).collect();

    let upper_bound = if path == "input" { 4000000 } else { 20 };
    part2(input, upper_bound);

    // visualize_grid(input);
}

fn visualize_grid(input: Vec<((i32, i32), (i32, i32))>) {
    let mut grid = vec![vec!['.'; 400]; 400];
    for (sensor, beacon) in input.iter() {
        let divider = 10000;
        if sensor.0 >= 0 && sensor.1 >= 0 {
            let sx = (sensor.0 / divider) as usize;
            let sy = (sensor.1 / divider) as usize;
            grid[sy][sx] = 'S';
        }
        if beacon.0 >= 0 && beacon.1 >= 0 {
            let bx = (beacon.0 / divider) as usize;
            let by = (beacon.1 / divider) as usize;
            grid[by][bx] = 'B';
        }
    }
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn part2(input: Vec<((i32, i32), (i32, i32))>, upper_bound: i32) {
    let (min_x, max_x, min_y, max_y) = get_bounding_box(&input, upper_bound);
    // let min_y = 2000000;
    // let max_y = 2500000;
    // let min_y = 0;
    // let max_y = 20;
    // let min_y = 0;
    // let max_y = 1000;

    for (sensor, beacon) in input.iter() {
        let dist = get_manhattan_distance(sensor, beacon);
        for (x, y) in get_outer_perimeter(sensor, dist) {
            if 0 <= x && x < upper_bound && 0 <= y && y <= upper_bound {
                guess_cell(&input, x, y);
            }
        }
    }
}

fn get_outer_perimeter(sensor: &(i32, i32), dist: i32) -> Vec<(i32, i32)> {
    let mut perimeter = vec![];

    // far right
    let mut x = dist + 1;
    let mut y = 0;

    // towards bottom
    while x > 0 {
        perimeter.push((sensor.0 + x, sensor.1 + y));
        x -= 1;
        y -= 1;
    }

    // towards left
    while y < 0 {
        perimeter.push((sensor.0 + x, sensor.1 + y));
        x -= 1;
        y += 1;
    }

    // towards top
    while x < 0 {
        perimeter.push((sensor.0 + x, sensor.1 + y));
        x += 1;
        y += 1;
    }

    // towards right
    while y > 0 {
        perimeter.push((sensor.0 + x, sensor.1 + y));
        x += 1;
        y -= 1;
    }

    perimeter
}

fn guess_cell(input: &Vec<((i32, i32), (i32, i32))>, x: i32, y: i32) -> i32 {
    for (sensor, beacon) in input.iter() {
        if beacon == &(x, y) {
            return 1;
        }

        let dist1 = get_manhattan_distance(sensor, beacon);

        if sensor == &(x, y) {
            return dist1 + 1;
        }

        let dist2 = get_manhattan_distance(sensor, &(x, y));
        if dist1 >= dist2 {
            // println!("{:?} {:?} {} {}", (x, y), sensor, dist1, dist2);
            let dx = sensor.0 - x;
            // let dy = (sensor.1 - y).abs();
            if dx > 0 {
                return dx * 2 + 1;
            }
            // return dy + dx + 1;
            return 1;
        }
    }

    let tuning_frequency = x as i64 * 4000000 + y as i64;
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
