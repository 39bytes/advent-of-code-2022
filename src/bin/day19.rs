use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let blueprints = parse_input("inputs/day19.txt");

    println!("Solution to part 1: {}", part1(&blueprints));
    println!("Solution to part 2: {}", part2(&blueprints));
}

fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    let mut sum_quality = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut dp = HashMap::new();
        let geodes = test_blueprint(
            &mut dp,
            blueprint,
            ResourceState {
                ore: 0,
                ore_robots: 1,
                clay: 0,
                clay_robots: 0,
                obsidian: 0,
                obsidian_robots: 0,
                geode: 0,
                geode_robots: 0,
            },
            24,
            (0, 0, 0, 0),
        );
        sum_quality += (i + 1) as u32 * geodes;
    }
    sum_quality
}

fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    let mut result = 1;
    for blueprint in blueprints.iter().take(3) {
        let mut dp = HashMap::new();
        let geodes = test_blueprint(
            &mut dp,
            blueprint,
            ResourceState {
                ore: 0,
                ore_robots: 1,
                clay: 0,
                clay_robots: 0,
                obsidian: 0,
                obsidian_robots: 0,
                geode: 0,
                geode_robots: 0,
            },
            32,
            (0, 0, 0, 0),
        );
        result *= geodes;
    }
    result
}

fn test_blueprint(
    dp: &mut HashMap<ResourceState, u32>,
    blueprint: &Blueprint,
    state: ResourceState,
    minutes_left: u32,
    new_robots: (u32, u32, u32, u32),
) -> u32 {
    if minutes_left == 0 {
        return state.geode;
    }

    if let Some(&amount) = dp.get(&state) {
        return amount;
    }

    let state = ResourceState {
        ore: state.ore + state.ore_robots,
        clay: state.clay + state.clay_robots,
        obsidian: state.obsidian + state.obsidian_robots,
        geode: state.geode + state.geode_robots,
        ore_robots: state.ore_robots + new_robots.0,
        clay_robots: state.clay_robots + new_robots.1,
        obsidian_robots: state.obsidian_robots + new_robots.2,
        geode_robots: state.geode_robots + new_robots.3,
    };

    let mut results = Vec::new();
    if minutes_left > 1 {
        // Robots should take a minute to build, currently they start producing resources immediately..
        if state.ore >= blueprint.ore_robot_cost
            && (state.ore_robots * minutes_left + state.ore)
                < ([
                    blueprint.clay_robot_cost,
                    blueprint.obsidian_robot_cost.0,
                    blueprint.geode_robot_cost.0,
                ]
                .iter()
                .max()
                .unwrap()
                    * minutes_left)
        {
            // Build ore robot
            results.push(test_blueprint(
                dp,
                blueprint,
                ResourceState {
                    ore: state.ore - blueprint.ore_robot_cost,
                    ..state
                },
                minutes_left - 1,
                (1, 0, 0, 0),
            ))
        }

        if state.ore >= blueprint.clay_robot_cost
            && (state.clay_robots * minutes_left + state.clay)
                < (blueprint.obsidian_robot_cost.1 * minutes_left)
        {
            // Build clay robot
            results.push(test_blueprint(
                dp,
                blueprint,
                ResourceState {
                    ore: state.ore - blueprint.clay_robot_cost,
                    ..state
                },
                minutes_left - 1,
                (0, 1, 0, 0),
            ))
        }

        if state.ore >= blueprint.obsidian_robot_cost.0
            && state.clay >= blueprint.obsidian_robot_cost.1
            && (state.obsidian_robots * minutes_left + state.obsidian)
                < (blueprint.geode_robot_cost.1 * minutes_left)
        {
            // Build obsidian robot
            results.push(test_blueprint(
                dp,
                blueprint,
                ResourceState {
                    ore: state.ore - blueprint.obsidian_robot_cost.0,
                    clay: state.clay - blueprint.obsidian_robot_cost.1,
                    ..state
                },
                minutes_left - 1,
                (0, 0, 1, 0),
            ))
        }

        if state.ore >= blueprint.geode_robot_cost.0
            && state.obsidian >= blueprint.geode_robot_cost.1
        {
            // Build geode robot
            results.push(test_blueprint(
                dp,
                blueprint,
                ResourceState {
                    ore: state.ore - blueprint.geode_robot_cost.0,
                    obsidian: state.obsidian - blueprint.geode_robot_cost.1,
                    ..state
                },
                minutes_left - 1,
                (0, 0, 0, 1),
            ))
        }
    }

    // Do nothing
    results.push(test_blueprint(
        dp,
        blueprint,
        ResourceState { ..state },
        minutes_left - 1,
        (0, 0, 0, 0),
    ));

    let best = *results.iter().max().unwrap_or(&0);
    dp.insert(state, best);
    best
}

fn parse_input(path: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                blueprints.push(parse_input_line(line));
            }
        }
    }

    blueprints
}

fn parse_input_line(line: String) -> Blueprint {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    }

    let caps = RE.captures(&line).unwrap();
    let costs: Vec<u32> = caps
        .iter()
        .map(|x| x.unwrap().as_str().parse::<u32>().unwrap_or(0))
        .collect();

    Blueprint {
        ore_robot_cost: costs[1],
        clay_robot_cost: costs[2],
        obsidian_robot_cost: (costs[3], costs[4]),
        geode_robot_cost: (costs[5], costs[6]),
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct ResourceState {
    ore: u32,
    ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    geode: u32,
    geode_robots: u32,
}
