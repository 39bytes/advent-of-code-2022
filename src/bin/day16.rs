use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path;

fn main() {
    part1();
}

fn part1() {
    let (valves, mut flow_valves) = parse_input();

    let mut flow_per_min = 0;
    let mut pressure_released = 0;
    let mut minutes_left: u32 = 30;

    // Store visited flow valves in Vec

    let mut cur_valve = valves.get("AA").unwrap();
    flow_valves.push(String::from("AA"));

    let weighted_valves = get_weighted_nodes_list(valves, flow_valves);
    let mut remaining = weighted_valves.clone();
    remaining.remove("AA");
    let max_pressure = dfs(&weighted_valves, remaining.clone(), "AA", 30);

    // let mut human_remaining: HashMap<String, WeightedNode> = HashMap::new();
    // let keys: Vec<String> = remaining.keys().map(|x| x.clone()).collect();

    // let pressures =
    // for key in keys {
    //     human_remaining.insert(key.clone(), remaining.remove(&key).unwrap());
    // }

    // let (max_human, remaining_valves) = dfs(&weighted_valves, remaining.clone(), "AA", 26);
    // println!("{:?}", remaining_valves);
    // let (max_elephant, _) = dfs(&weighted_valves, remaining_valves, "AA", 26);

    println!("Solution to part 1: {}", max_pressure);
    let max_pressure2 = dfs2(&weighted_valves, remaining.clone(), ("AA", "AA"), (26, 26));
    println!("Solution to part 2: {}", max_pressure2);
}

fn get_weighted_nodes_list(
    valves: HashMap<String, Valve>,
    flow_valves: Vec<String>,
) -> HashMap<String, WeightedNode> {
    let mut weighted_valves: HashMap<String, WeightedNode> = HashMap::new();

    for valve in flow_valves.iter() {
        let mut queue: VecDeque<&Valve> = VecDeque::new();
        let mut visited: Vec<&str> = Vec::new();
        let mut prev: HashMap<&str, &str> = HashMap::new();

        let mut weighted_adjacents: HashMap<String, u32> = HashMap::new();

        let valve = valves.get(valve).unwrap();
        queue.push_back(valve);
        visited.push(&valve.name);

        while let Some(v) = queue.pop_front() {
            for adj in &v.adjacent {
                if !visited.contains(&adj.as_str()) {
                    visited.push(&adj);
                    prev.insert(adj, &v.name);
                    queue.push_back(valves.get(adj).unwrap());

                    if flow_valves.contains(adj) {
                        let mut path_len = 0;
                        let mut cur = adj.as_str();
                        while let Some(u) = prev.get(cur) {
                            path_len += 1;
                            cur = u;
                        }
                        weighted_adjacents.insert(adj.to_string(), path_len);
                    }
                }
            }
        }
        weighted_valves.insert(
            valve.name.clone(),
            WeightedNode {
                name: valve.name.clone(),
                flow_rate: valve.flow_rate,
                adjacent: weighted_adjacents,
            },
        );
    }

    weighted_valves
}

fn dfs(
    weighted_valves: &HashMap<String, WeightedNode>,
    remaining: HashMap<String, WeightedNode>,
    current: &str,
    minutes_left: u32,
) -> u32 {
    let mut results: Vec<u32> = Vec::new();

    for (name, valve) in &remaining {
        let current_valve = weighted_valves.get(current).unwrap();
        let distance = current_valve.adjacent.get(name).unwrap();

        if distance >= &minutes_left {
            continue;
        }

        let minutes_left = minutes_left - distance - 1;
        let pressure = valve.flow_rate * minutes_left;

        let mut remaining_valves: HashMap<String, WeightedNode> = remaining.clone();
        remaining_valves.remove(name);

        let best = pressure + dfs(&weighted_valves, remaining_valves, name, minutes_left);

        results.push(best);
    }
    *results.iter().max().unwrap_or(&0)
}

fn dfs2(
    weighted_valves: &HashMap<String, WeightedNode>,
    remaining: HashMap<String, WeightedNode>,
    current: (&str, &str),
    minutes_left: (u32, u32),
) -> u32 {
    let mut results: Vec<u32> = Vec::new();

    let mut remaining = remaining.clone();
    remaining.remove(current.0);
    remaining.remove(current.1);

    for (human_valve_name, human_valve) in &remaining {
        if current.0 == "AA" {
            println!("{human_valve_name}");
        }
        let current_human_valve = weighted_valves.get(current.0).unwrap();
        let human_distance = current_human_valve.adjacent.get(human_valve_name).unwrap();

        if human_distance >= &minutes_left.0 {
            continue;
        }

        let human_minutes_left = minutes_left.0 - human_distance - 1;
        let human_pressure = human_valve.flow_rate * human_minutes_left;
        for (elephant_valve_name, elephant_valve) in &remaining {
            if elephant_valve_name == human_valve_name {
                continue;
            }

            let current_elephant_valve = weighted_valves.get(current.1).unwrap();
            let elephant_distance = current_elephant_valve
                .adjacent
                .get(elephant_valve_name)
                .unwrap();

            if elephant_distance >= &minutes_left.1 {
                continue;
            }

            let elephant_minutes_left = minutes_left.1 - elephant_distance - 1;
            let elephant_pressure = elephant_valve.flow_rate * elephant_minutes_left;

            let best = human_pressure
                + elephant_pressure
                + dfs2(
                    weighted_valves,
                    remaining.clone(),
                    (human_valve_name, elephant_valve_name),
                    (human_minutes_left, elephant_minutes_left),
                );

            results.push(best);
        }
    }
    *results.iter().max().unwrap_or(&0)
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    adjacent: Vec<String>,
}

#[derive(Debug, Clone)]
struct WeightedNode {
    name: String,
    flow_rate: u32,
    adjacent: HashMap<String, u32>,
}

fn parse_input() -> (HashMap<String, Valve>, Vec<String>) {
    let file = File::open("inputs/day16.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut flow_valves: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let valve = parse_input_line(line);
            if valve.flow_rate > 0 {
                flow_valves.push(valve.name.clone());
            }
            valves.insert(valve.name.clone(), valve);
        }
    }
    (valves, flow_valves)
}

fn parse_input_line(line: String) -> Valve {
    let parts: Vec<&str> = line.split("; ").collect();
    let valve_part: Vec<&str> = parts[0].split_whitespace().collect();

    let valve_name = valve_part[1].to_string();
    let flow_rate: u32 = valve_part[4].split("=").last().unwrap().parse().unwrap();

    let adjacent_valves: &str = match parts[1].strip_prefix("tunnels lead to valves ") {
        Some(val) => val,
        None => parts[1].strip_prefix("tunnel leads to valve ").unwrap(),
    };

    let adjacent: Vec<String> = if adjacent_valves.contains(",") {
        adjacent_valves
            .split(",")
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        vec![adjacent_valves.to_string()]
    };

    Valve {
        name: valve_name,
        flow_rate,
        adjacent,
    }
}
