use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    str::FromStr,
};
// use aoc_utils::submit;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq)]
struct Valve {
    name: String,
    flow_rate: u32,
    lead_to: Vec<String>,
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.flow_rate.partial_cmp(&other.flow_rate)
    }
}

impl FromStr for Valve {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(' ');
        let mut s = s.skip(1);
        let name = s.next().unwrap().to_string();
        let mut s = s.skip(2);
        let flow_rate = s.next().unwrap().trim_end_matches(';')[5..]
            .parse()
            .unwrap();
        let lead_to = s
            .skip(4)
            .map(|s| s.trim_end_matches(',').to_string())
            .collect();

        Ok(Valve {
            name,
            flow_rate,
            lead_to,
        })
    }
}

fn main() {
    // let path = "input";
    let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| line.parse::<Valve>().unwrap())
        .collect();

    let mut valves = HashMap::new();
    let first_valve_name = input[0].name.clone();
    for valve in input {
        valves.insert(valve.name.clone(), valve);
    }

    println!("{:?}", valves);
    part1(&valves, first_valve_name);

    // println!("{:?}", &input[0..5]);
    // submit("1", false);
}

fn part1(valves: &HashMap<String, Valve>, first_valve: String) {
    let mut minutes_left = 31;
    let mut pressure_release = 0;

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let valve = valves.get(&first_valve).unwrap();
    queue.push(valve);

    while let Some(valve) = queue.pop() {
        if seen.contains(&valve.name) {
            continue;
        }
        seen.insert(&valve.name);
        minutes_left -= 1;

        if valve.flow_rate != 0 {
            minutes_left -= 1;
            pressure_release += valve.flow_rate * minutes_left;
        }
        println!("{} {}min * {}", valve.name, minutes_left, valve.flow_rate);

        for tunnel in valve.lead_to.iter() {
            queue.push(valves.get(tunnel).unwrap());
        }

        if minutes_left <= 0 {
            break;
        }
    }

    println!("{}", pressure_release);
}
