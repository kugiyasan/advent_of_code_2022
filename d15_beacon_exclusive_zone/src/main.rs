// use aoc_utils::submit;
use std::fs;

type Vec2 = (i32, i32);

fn main() {
    let path = "input";
    // let path = "example";

    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<(Vec2, Vec2)> = buf.trim().split('\n').map(parse_line).collect();

    let upper_bound = if path == "input" { 4000000 } else { 20 };

    visualize_grid(&input);
    part2(input, upper_bound);
}

fn visualize_grid(input: &[(Vec2, Vec2)]) {
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

fn part2(input: Vec<(Vec2, Vec2)>, upper_bound: i32) {
    for (sensor, beacon) in input.iter() {
        let dist = get_manhattan_distance(sensor, beacon);
        for (x, y) in get_outer_perimeter(sensor, dist) {
            if 0 <= x && x < upper_bound && 0 <= y && y <= upper_bound {
                guess_cell(&input, x, y);
            }
        }
    }
}

fn get_outer_perimeter(sensor: &Vec2, dist: i32) -> Vec<Vec2> {
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

fn guess_cell(input: &[(Vec2, Vec2)], x: i32, y: i32) {
    for (sensor, beacon) in input.iter() {
        if beacon == &(x, y) {
            return;
        }

        if sensor == &(x, y) {
            return;
        }

        let dist1 = get_manhattan_distance(sensor, beacon);
        let dist2 = get_manhattan_distance(sensor, &(x, y));
        if dist1 >= dist2 {
            return;
        }
    }

    let tuning_frequency = x as i64 * 4000000 + y as i64;
    println!("x: {} y: {} tuning_frequency: {}", x, y, tuning_frequency);
}

fn parse_line(line: &str) -> (Vec2, Vec2) {
    let coords = line[10..]
        .split(": closest beacon is at ")
        .map(parse_coord)
        .collect::<Vec<_>>();

    (coords[0], coords[1])
}

fn parse_coord(coord: &str) -> Vec2 {
    let coord = coord
        .split(", ")
        .map(|n| n[2..].parse().unwrap())
        .collect::<Vec<_>>();

    (coord[0], coord[1])
}

fn get_manhattan_distance((x1, y1): &Vec2, (x2, y2): &Vec2) -> i32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx.abs() + dy.abs()
}
