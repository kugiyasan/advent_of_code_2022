use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        match s.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => Ok(Instruction::Addx(
                s.next().unwrap().parse().map_err(|_| ())?,
            )),
            _ => Err(()),
        }
    }
}

struct Crt {
    screen: [[char; 40]; 6],
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: [['\0'; 40]; 6],
        }
    }

    fn draw_pixel(&mut self, cycle: i32, x_register: i32) {
        let cycle = cycle % (40 * 6);
        let y = cycle as usize / 40;
        let x = cycle as usize % 40;

        if (x_register - x as i32).abs() < 2 {
            self.screen[y][x] = '#';
        } else {
            self.screen[y][x] = '.';
        }
    }
    fn print(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", self.screen[y][x]);
            }
            println!();
        }
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<Instruction> = buf
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut cycle = 0;
    let mut x_register = 1;
    let mut sum = 0;
    let mut crt = Crt::new();

    for instruction in input.iter() {
        match instruction {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(x) => {
                cycle += 1;
                sum += check_signal_strength(x_register, cycle);
                crt.draw_pixel(cycle, x_register);
                cycle += 1;
                x_register += x;
            }
        }
        sum += check_signal_strength(x_register, cycle);
        crt.draw_pixel(cycle, x_register);
        if cycle % (40 * 6) == 0 {
            crt.print();
        }
    }

    println!("{:?}", &input[0..5]);
    println!("{}", sum);
}

fn check_signal_strength(signal_strength: i32, cycle: i32) -> i32 {
    if (cycle + 20) % 40 == 0 {
        println!(
            "{} * {} = {}",
            signal_strength,
            cycle,
            signal_strength * cycle
        );
        signal_strength * cycle
    } else {
        0
    }
}
