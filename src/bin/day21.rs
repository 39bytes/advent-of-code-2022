use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = parse_input("inputs/day21.txt");

    println!("Solution to part 1: {}", part1(&lines));
    println!("Solution to part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut memo = HashMap::new();
    get_monkey_val(&mut memo, lines, "root")
}

fn part2(lines: &Vec<String>) -> i64 {
    let names: Vec<&str> = get_monkey_line(lines, "root")
        .expect("Root not found")
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .collect();

    let mut low = 0;
    let mut high = 2_i64.pow(48);

    let mut results: Vec<i64> = Vec::new();

    while low <= high {
        let middle = (low + high) / 2;

        let mut memo1 = HashMap::new();
        memo1.insert(String::from("humn"), middle);
        let mut memo2 = HashMap::new();
        memo2.insert(String::from("humn"), middle);

        let val1 = get_monkey_val(&mut memo1, lines, names[0]);
        let val2 = get_monkey_val(&mut memo2, lines, names[2]);

        let diff = val1 - val2;

        if diff > 0 {
            low = middle + 1;
        } else if diff < 0 {
            high = middle - 1;
        } else {
            results.push(middle);
            high = middle - 1;
        }
    }
    results[1]
}

fn get_monkey_val(memo: &mut HashMap<String, i64>, lines: &Vec<String>, name: &str) -> i64 {
    if let Some(val) = memo.get(name) {
        return *val;
    }

    let line = get_monkey_line(lines, name).expect("Monkey name not found");
    let exp = line.split_once(": ").unwrap().1;

    if let Ok(val) = exp.parse::<i64>() {
        memo.insert(name.to_string(), val);
        return val;
    }

    let parts: Vec<&str> = exp.split(" ").collect();

    let result = match parts[1] {
        "+" => get_monkey_val(memo, lines, parts[0]) + get_monkey_val(memo, lines, parts[2]),
        "-" => get_monkey_val(memo, lines, parts[0]) - get_monkey_val(memo, lines, parts[2]),
        "*" => get_monkey_val(memo, lines, parts[0]) * get_monkey_val(memo, lines, parts[2]),
        "/" => get_monkey_val(memo, lines, parts[0]) / get_monkey_val(memo, lines, parts[2]),
        _ => panic!("Invalid operator"),
    };

    memo.insert(name.to_string(), result);
    return result;
}

fn get_monkey_line<'a>(lines: &'a Vec<String>, name: &str) -> Option<&'a str> {
    for line in lines {
        if line.starts_with(name) {
            return Some(line);
        }
    }
    None
}

fn parse_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().filter_map(|l| l.ok()).collect()
}
