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
            .skip(1)
            .next()
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

    print_monkeys(&monkeys);

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let items = mem::replace(&mut monkeys[i].items, Vec::new());
            inspections[i] += items.len();
            for mut item in items {
                let monkey = &mut monkeys[i];
                item = (monkey.operation)(item) / 3;

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
        print_monkeys(&monkeys);
    }

    let result: Vec<_> = monkeys.into_iter().map(|m| m.items).collect();
    println!("{:?}", result);
    println!("{:?}", inspections);
    inspections.sort();
    let values = inspections.iter().rev().take(2).collect::<Vec<_>>();
    println!("{}", values[0] * values[1])
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    let monkeys = monkeys.iter().map(|m| &m.items).collect::<Vec<_>>();
    for monkey in monkeys {
        println!("{:?}", monkey);
    }

    println!();
}
