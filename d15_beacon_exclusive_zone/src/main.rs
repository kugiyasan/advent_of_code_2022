// use aoc_utils::submit;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<((i32, i32), (i32, i32))> = buf.trim().split('\n').map(parse_line).collect();

    let (min_x, max_x, min_y, max_y) = get_bounding_box(&input);
    let min_x = min_x - 10000000;
    let max_x = max_x + 10000000;

    let y = if path == "input" { 2000000 } else { 10 };

    let mut count = 0;
    let mut line = vec!['.'; (max_x - min_x + 1) as usize];

    for x in min_x..=max_x {
        for (sensor, beacon) in input.iter() {
            if sensor == &(x, y) {
                line[(x - min_x) as usize] = 'S';
                continue;
            }

            if beacon == &(x, y) {
                line[(x - min_x) as usize] = 'B';
                continue;
            }

            let dist1 = get_manhattan_distance(sensor, beacon);
            let dist2 = get_manhattan_distance(sensor, &(x, y));
            // println!("{:?} {:?} {:?} {} {}", (x, y), sensor, beacon, dist1, dist2);
            if dist1 >= dist2 {
                count += 1;
                line[(x - min_x) as usize] = '#';
                break;
            }
        }
    }

    // println!("{:?}", &input[0..5]);
    println!("min_x: {} max_x: {} min_y: {} max_y: {}", min_x, max_x, min_y, max_y);
    println!("{}", count);
    // println!("{:?}", line.iter().collect::<String>());
    // submit("1", false);
}

fn get_bounding_box(input: &Vec<((i32, i32), (i32, i32))>) -> (i32, i32, i32, i32) {
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
