use std::{fs, mem, str::FromStr};

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: [i64; 3],
}

impl Monkey {
    fn get_operation(i: usize) -> Box<dyn Fn(i64) -> i64> {
        let operations = [
            |x| x * 7,
            |x| x + 4,
            |x| x + 2,
            |x| x + 7,
            |x| x * 17,
            |x| x + 8,
            |x| x + 6,
            |x| x * x,
        ];
        // let operations = [|x| x * 19, |x| x + 6, |x| x * x, |x| x + 3];
        Box::new(operations[i])
    }
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('\n');
        let i = s
            .next()
            .unwrap()
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        let items = s
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        s.next().unwrap();
        let operation = Self::get_operation(i);

        let test = s
            .map(|line| line.split(' ').last().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self {
            items,
            operation,
            test,
        })
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let mut monkeys: Vec<_> = buf
        .trim()
        .split("\n\n")
        .map(|block| block.parse::<Monkey>().unwrap())
        .collect();

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test[0])
        .reduce(|acc, x| acc * x)
        .unwrap();

    print_monkeys(&monkeys);
    println!("lcm: {}", lcm);

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[i].items);
            inspections[i] += items.len();
            for mut item in items {
                let monkey = &mut monkeys[i];
                item = (monkey.operation)(item) % lcm;

                let divider = monkey.test[0];
                let true_throw = monkey.test[1] as usize;
                let false_throw = monkey.test[2] as usize;

                if item % divider == 0 {
                    monkeys[true_throw].items.push(item);
                } else {
                    monkeys[false_throw].items.push(item);
                }
            }
        }
        // print_monkeys(&monkeys);
    }

    print_monkeys(&monkeys);
    println!("{:?}", inspections);
    inspections.sort();
    let value = inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|acc, x| acc * x)
        .unwrap();
    println!("{}", value)
}

fn print_monkeys(monkeys: &[Monkey]) {
    let monkeys = monkeys.iter().map(|m| &m.items).collect::<Vec<_>>();
    for monkey in monkeys {
        println!("{:?}", monkey);
    }

    println!();
}
