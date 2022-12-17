extern crate pathfinding;
use pathfinding::prelude::dfs_reach;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};
// use aoc_utils::submit;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq)]
struct Valve {
    name: String,
    flow_rate: i32,
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

    let valves_indexes = valves.values().collect::<Vec<_>>();

    let mut useful_valves = Vec::new();
    for valve in valves.values() {
        if valve.flow_rate != 0 {
            useful_valves.push(valve.name.clone());
        }
    }
    println!("{:#?}", useful_valves);

    let mut distance_matrix = vec![vec![-1; useful_valves.len()]; useful_valves.len()];
    for (i, valve1) in useful_valves.iter().enumerate() {
        for valve2 in useful_valves[i + 1..].iter() {
            let distance = find_distance(&valves, &mut HashSet::new(), valve1, valve2);
            let y = useful_valves.iter().position(|v| v == valve1).unwrap();
            let x = useful_valves.iter().position(|v| v == valve2).unwrap();
            distance_matrix[y][x] = distance;
        }
    }
    println!("{:?}", distance_matrix);
    let dfs_reachable = dfs_reach(&first_valve_name, |&v| {
        let v = valves.get(v).unwrap();
        v.lead_to.iter()
    });

    for i in dfs_reachable {
        println!("{:?}", i);
    }

    // let pressure_release = dfs(&valves, &first_valve_name, 31, 0);
    // println!("{}", pressure_release);

    // println!("{:?}", &input[0..5]);
    // submit("1", false);
}

fn find_distance(
    valves: &HashMap<String, Valve>,
    seen: &mut HashSet<String>,
    valve1: &str,
    valve2: &str,
) -> i32 {
    let valve1 = valves.get(valve1).unwrap();
    let valve2 = valves.get(valve2).unwrap();

    if valve1.name == valve2.name {
        return 0;
    }

    if valve1.lead_to.contains(&valve2.name) {
        return 1;
    }

    seen.insert(valve1.name.clone());
    valve1
        .lead_to
        .iter()
        .filter_map(|v| {
            if !seen.contains(v) {
                Some(find_distance(valves, seen, v, &valve2.name))
            } else {
                None
            }
        })
        .min()
        .unwrap_or(-1)
        + 1
}

fn my_dfs(
    valves: &HashMap<String, Valve>,
    current_valve_name: &str,
    minutes_left: i32,
    pressure_release: i32,
) -> i32 {
    if minutes_left <= 0 {
        return pressure_release;
    }
    println!(
        "dfs {} {} {}",
        current_valve_name, minutes_left, pressure_release
    );
    let valve = valves.get(current_valve_name).unwrap();

    let mut biggest_child_pressure = 0;

    if valve.flow_rate != 0 {
        let pressure = pressure_release + valve.flow_rate * minutes_left;
        for valve_name in valve.lead_to.iter() {
            let child_pressure = my_dfs(valves, valve_name, minutes_left - 2, pressure);
            biggest_child_pressure = biggest_child_pressure.max(child_pressure);
        }
    }

    for valve_name in valve.lead_to.iter() {
        let child_pressure = my_dfs(valves, valve_name, minutes_left - 1, pressure_release);
        biggest_child_pressure = biggest_child_pressure.max(child_pressure);
    }

    pressure_release + biggest_child_pressure
}
