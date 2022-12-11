use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LCM: u64 = 2 * 13 * 3 * 17 * 19 * 7 * 11 * 5;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkeys: Vec<Monkey> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let start_items = parse_starting_items(lines.next().unwrap().unwrap());
        let operation = parse_operation(lines.next().unwrap().unwrap());
        let test = parse_test(lines.next().unwrap().unwrap());
        let true_monkey = parse_monkey_num(lines.next().unwrap().unwrap());
        let false_monkey = parse_monkey_num(lines.next().unwrap().unwrap());

        monkeys.push(Monkey {
            items: start_items,
            operation,
            test,
            true_monkey,
            false_monkey,
            times_inspected: 0,
        });
        lines.next();
    }

    let mut round = 1;
    while round <= 20 {
        for i in 0..monkeys.len() {
            loop {
                let item = { monkeys[i].items.pop_front() };
                if item.is_none() {
                    break;
                }
                let (new, test_result, true_monkey, false_monkey) = {
                    let item = item.unwrap();
                    let new = (monkeys[i].operation)(item) / 3;
                    let test_result = (monkeys[i].test)(new);
                    (
                        new,
                        test_result,
                        monkeys[i].true_monkey,
                        monkeys[i].false_monkey,
                    )
                };
                if test_result {
                    monkeys[true_monkey].items.push_back(new);
                } else {
                    monkeys[false_monkey].items.push_back(new);
                }
                monkeys[i].times_inspected += 1;
            }
        }
        round += 1;
    }

    monkeys.sort_by_key(|m| m.times_inspected);

    let monkey_business =
        monkeys[monkeys.len() - 1].times_inspected * monkeys[monkeys.len() - 2].times_inspected;
    println!("Solution to part 1: {}", monkey_business);
}

fn part2() {
    let file = File::open("inputs/day11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkeys: Vec<Monkey> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let start_items = parse_starting_items(lines.next().unwrap().unwrap());
        let operation = parse_operation(lines.next().unwrap().unwrap());
        let test = parse_test(lines.next().unwrap().unwrap());
        let true_monkey = parse_monkey_num(lines.next().unwrap().unwrap());
        let false_monkey = parse_monkey_num(lines.next().unwrap().unwrap());

        monkeys.push(Monkey {
            items: start_items,
            operation,
            test,
            true_monkey,
            false_monkey,
            times_inspected: 0,
        });
        lines.next();
    }

    let mut round = 1;
    while round <= 10000 {
        for i in 0..monkeys.len() {
            loop {
                let item = { monkeys[i].items.pop_front() };
                if item.is_none() {
                    break;
                }
                let (new, test_result, true_monkey, false_monkey) = {
                    let item = item.unwrap();
                    let new = (monkeys[i].operation)(item) % LCM;
                    let test_result = (monkeys[i].test)(new);
                    (
                        new,
                        test_result,
                        monkeys[i].true_monkey,
                        monkeys[i].false_monkey,
                    )
                };
                if test_result {
                    monkeys[true_monkey].items.push_back(new);
                } else {
                    monkeys[false_monkey].items.push_back(new);
                }
                monkeys[i].times_inspected += 1;
            }
        }
        round += 1;
    }

    monkeys.sort_by_key(|m| m.times_inspected);
    let monkey_business =
        monkeys[monkeys.len() - 1].times_inspected * monkeys[monkeys.len() - 2].times_inspected;

    println!("Solution to part 2: {}", monkey_business);
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
    times_inspected: u64,
}

fn parse_starting_items(s: String) -> VecDeque<u64> {
    s.trim()
        .strip_prefix("Starting items: ")
        .unwrap()
        .split(", ")
        .map(|x| x.parse::<u64>().expect("Failed to parse starting item."))
        .collect()
}

fn parse_operation(s: String) -> Box<dyn Fn(u64) -> u64> {
    let terms: Vec<&str> = s
        .trim()
        .strip_prefix("Operation: ")
        .unwrap()
        .split_whitespace()
        .collect();
    match terms[3] {
        // Operator
        "*" => match terms[4] {
            "old" => Box::new(|old| old * old),
            n => {
                let num = n.parse::<u64>().unwrap();
                Box::new(move |old| old * num)
            }
        },
        "+" => match terms[4] {
            "old" => Box::new(|old| old + old),
            n => {
                let num = n.parse::<u64>().unwrap();
                Box::new(move |old| old + num)
            }
        },
        _ => {
            panic!("Invalid operation.")
        }
    }
}

fn parse_test(s: String) -> Box<dyn Fn(u64) -> bool> {
    let divisible_by: u64 = s
        .trim()
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    Box::new(move |old| old % divisible_by == 0)
}

fn parse_monkey_num(s: String) -> usize {
    s.split_whitespace().last().unwrap().parse().unwrap()
}
