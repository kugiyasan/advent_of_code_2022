use std::{
    collections::{HashSet, VecDeque},
    fs,
};

struct Node {
    position: usize,
    parent: Option<usize>,
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<u8>, width: usize, height: usize) -> Self {
        Grid {
            grid,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y * self.width + x]
    }

    fn get_pos(&self, position: usize) -> (usize, usize) {
        // (x, y)
        (position % self.width, position / self.width)
    }

    fn get_char(&self, position: usize) -> char {
        Self::cell_to_char(self.grid[position])
    }

    fn cell_to_char(c: u8) -> char {
        match c {
            0 => 'S',
            27 => 'E',
            x => (x + b'a' - 1) as char,
        }
    }

    fn get_neighbors(&self, parent: &Node) -> Vec<Node> {
        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        let mut results = vec![];

        for neighbor in neighbors {
            let (x, y) = self.get_pos(parent.position);
            let x = x as i32 + neighbor.0;
            let y = y as i32 + neighbor.1;

            let width = self.width.try_into().unwrap();
            let height = self.height.try_into().unwrap();

            if x < 0 || x >= width || y < 0 || y >= height {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            // println!(
            //     "{:?} {} {}",
            //     (x, y),
            //     self.get_char(parent.position),
            //     self.get_char(y * self.width + x),
            // );
            let max_next_elevation = match self.grid[parent.position] {
                0 => 2,
                x => x + 1,
            };

            let next_elevation = match self.get(x, y) {
                27 => 26,
                x => x,
            };
            // let next_elevation = self.get(x, y);

            if max_next_elevation >= next_elevation {
                // println!("x {x} y {y}");
                results.push(Node {
                    position: y * self.width + x,
                    parent: Some(parent.position),
                });
            }
        }
        results
    }

    fn print_path(&self, path: &[usize]) {
        println!(
            "{:?}",
            path.iter().map(|&p| self.get_pos(p)).collect::<Vec<_>>()
        );

        for (y, row) in self.grid.chunks(self.width).enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let c = Self::cell_to_char(c);
                let position = y * self.width + x;

                if let Some(i) = path.iter().position(|&p| p == position) {
                    let color = (i / 8) % 6 + 1;
                    print!("\x1b[30m\x1b[4{}m{}\x1b[0m", color, c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let grid: Vec<_> = buf
        .lines()
        .flat_map(|line| {
            line.bytes()
                .map(|c| {
                    if c == b'S' {
                        0
                    } else if c == b'E' {
                        27
                    } else {
                        c - b'a' + 1
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    let height = buf.lines().count();
    let width = buf.lines().next().unwrap().len();
    println!("height {height} width {width}");

    let start_positions = grid
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c <= 1 { Some(i) } else { None })
        .collect::<Vec<_>>();

    let grid = Grid::new(grid, width, height);
    let mut seen = HashSet::new();
    let mut parents = vec![None; width * height];

    let mut queue = VecDeque::new();
    for start_pos in start_positions {
        queue.push_back(Node {
            position: start_pos,
            parent: None,
        });
    }

    while let Some(node) = queue.pop_front() {
        if seen.contains(&node.position) {
            continue;
        }
        seen.insert(node.position);
        parents[node.position] = node.parent;
        // println!("visiting {:?}", grid.get_pos(node.position));

        for neighbor in grid.get_neighbors(&node) {
            if grid.grid[neighbor.position] == 27 {
                let path = find_distance(&parents, neighbor.parent.unwrap());
                grid.print_path(&path);
                // println!("{:?}", parents);
                println!("FOUND PATH {}", path.len());
                return;
            }
            queue.push_back(neighbor);
        }
    }

    // println!("{:?}", &input[0..5]);
}

fn find_distance(parents: &[Option<usize>], mut position: usize) -> Vec<usize> {
    let mut path = vec![position];

    while let Some(p) = parents[position] {
        position = p;
        path.push(position);
    }

    path.reverse();
    path
}
