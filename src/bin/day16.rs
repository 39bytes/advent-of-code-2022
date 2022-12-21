use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (valves, mut flow_valves) = parse_input();
    flow_valves.insert(0, String::from("AA"));

    let graph = get_weighted_graph(valves, flow_valves);
    println!("Solution to part 1: {}", part1(&graph));
    println!("Solution to part 2: {}", part2(&graph));
}

fn part1(graph: &Vec<(String, WeightedNode)>) -> u32 {
    dfs(graph, u16::MAX, 1, 30)
}

fn part2(graph: &Vec<(String, WeightedNode)>) -> u32 {
    let mut highest = 0;
    for i in 0..u16::MAX {
        let pressure = dfs(graph, i, 1, 26) + dfs(graph, !i, 1, 26);
        if pressure > highest {
            highest = pressure;
        }
    }
    highest
}

fn get_weighted_graph(
    valves: HashMap<String, Valve>,
    flow_valves: Vec<String>,
) -> Vec<(String, WeightedNode)> {
    let mut weighted_valves: Vec<(String, WeightedNode)> = Vec::new();

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
        weighted_valves.push((
            valve.name.clone(),
            WeightedNode {
                flow_rate: valve.flow_rate,
                adjacent: weighted_adjacents,
            },
        ));
    }

    weighted_valves
}

fn dfs(
    graph: &Vec<(String, WeightedNode)>,
    remaining: u16,
    current: u16,
    minutes_left: u32,
) -> u32 {
    let mut results: Vec<u32> = Vec::new();

    let remaining = if current != 1 {
        remaining ^ current
    } else {
        remaining & (u16::MAX - 1)
    };
    let (_, cur_valve) = &graph[current.trailing_zeros() as usize];

    for (i, (name, valve)) in graph.iter().enumerate() {
        if remaining & (1 << i) != 0 {
            let distance = cur_valve.adjacent.get(name).unwrap();
            if distance >= &minutes_left {
                continue;
            }

            let minutes_left = minutes_left - distance - 1;
            let pressure = valve.flow_rate * minutes_left;

            let best = pressure + dfs(&graph, remaining, 1 << i, minutes_left);
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
