// use aoc_utils::submit;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Rock,
    Air,
    Sand,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    sand_spawn_point: (usize, usize),
    sand_count: u32,
}

impl Grid {
    fn new(min_width: usize, max_width: usize, height: usize) -> Self {
        let width = max_width - min_width + 3;
        Self {
            cells: vec![vec![Cell::Air; width]; height],
            sand_spawn_point: (500 - min_width + 1, 0),
            sand_count: 0,
        }
    }

    fn print(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == self.sand_spawn_point.0 && y == self.sand_spawn_point.1 {
                    print!("+");
                    continue;
                }
                let c = match cell {
                    Cell::Rock => "#",
                    Cell::Air => ".",
                    Cell::Sand => "o",
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn drop_sand(&mut self) -> bool {
        let (mut x, mut y) = self.sand_spawn_point;
        loop {
            if y >= self.cells.len() - 1 {
                return true;
            }

            if self.cells[y + 1][x] == Cell::Air {
                y += 1;
            } else if self.cells[y + 1][x - 1] == Cell::Air {
                y += 1;
                x -= 1;
            } else if self.cells[y + 1][x + 1] == Cell::Air {
                y += 1;
                x += 1;
            } else {
                self.cells[y][x] = Cell::Sand;
                self.sand_count += 1;
                return false;
            }
        }
    }

    fn run(&mut self) -> u32 {
        while !self.drop_sand() {}
        self.sand_count
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Vec<(usize, usize)>> = buf
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let mut coords = coords.split(',');
                    (
                        coords.next().unwrap().parse().unwrap(),
                        coords.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let min_width = *input
        .iter()
        .map(|shape| shape.iter().map(|(x, _y)| x).min().unwrap())
        .min()
        .unwrap();
    let max_width = *input
        .iter()
        .map(|shape| shape.iter().map(|(x, _y)| x).max().unwrap())
        .max()
        .unwrap();
    let max_height = *input
        .iter()
        .map(|shape| shape.iter().map(|(_x, y)| y).max().unwrap())
        .max()
        .unwrap()
        + 1;

    let mut grid = Grid::new(min_width, max_width, max_height);

    let input: Vec<Vec<(usize, usize)>> = input
        .into_iter()
        .map(|shape| {
            shape
                .into_iter()
                .map(|(x, y)| (x - min_width + 1, y))
                .collect()
        })
        .collect();

    for shape in input.iter() {
        for arr in shape.windows(2) {
            if let &[(x1, y1), (x2, y2)] = arr {
                if x1 == x2 {
                    for y in y1..=y2 {
                        grid.cells[y][x1] = Cell::Rock;
                    }
                    for y in y2..=y1 {
                        grid.cells[y][x1] = Cell::Rock;
                    }
                } else if y1 == y2 {
                    for x in x1..=x2 {
                        grid.cells[y1][x] = Cell::Rock;
                    }
                    for x in x2..=x1 {
                        grid.cells[y1][x] = Cell::Rock;
                    }
                }
            }
        }
    }

    grid.print();
    let count = grid.run();
    grid.print();
    println!("count: {}", count);
    // println!("{:?}", &input[0..5]);
    // println!("{:?}", input);
    // submit("1", false);
}
