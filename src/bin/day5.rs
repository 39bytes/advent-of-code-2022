use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut stacks = parse_initial_stacks(&mut lines);
    lines.next();

    for line in lines {
        if let Ok(line) = line {
            let instruction: Vec<&str> = line.split(' ').collect();

            let amount: usize = instruction[1].parse().unwrap();
            let from = instruction[3].parse::<usize>().unwrap() - 1;
            let to = instruction[5].parse::<usize>().unwrap() - 1;

            for _ in 0..amount {
                let c = { stacks[from].pop().unwrap() };
                stacks[to].push(c);
            }
        }
    }

    let top: String = stacks.iter().map(|x| *x.last().unwrap()).collect();
    println!("Solution to part 1: {}", top);
}

fn part2() {
    let file = File::open("inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut stacks = parse_initial_stacks(&mut lines);
    lines.next();

    for line in lines {
        if let Ok(line) = line {
            let instruction: Vec<&str> = line.split(' ').collect();

            let amount: usize = instruction[1].parse().unwrap();
            let from = instruction[3].parse::<usize>().unwrap() - 1;
            let to = instruction[5].parse::<usize>().unwrap() - 1;

            let mut popped_crates = {
                let from_stack = &mut stacks[from];
                from_stack.split_off(from_stack.len() - amount)
            };

            let to_stack = &mut stacks[to];
            to_stack.append(&mut popped_crates);
        }
    }

    let top: String = stacks.iter().map(|x| *x.last().unwrap()).collect();
    println!("Solution to part 2: {}", top);
}

fn parse_initial_stacks<B>(lines: &mut Lines<B>) -> Vec<Vec<char>>
where
    B: BufRead,
{
    let stack_lines: Vec<String> = lines
        .by_ref()
        .take_while(|x| x.as_ref().unwrap().starts_with('['))
        .map(|r| r.unwrap())
        .collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let first = &stack_lines[0];
    for _ in 0..partition(first, 4).len() {
        stacks.push(vec![]);
    }

    for line in stack_lines {
        let segments = partition(&line, 4);
        for (i, c) in segments.iter().enumerate() {
            if !c.trim().is_empty() {
                stacks[i].push(c.chars().nth(1).unwrap());
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn partition(s: &str, n: usize) -> Vec<String> {
    let mut segments: Vec<String> = Vec::new();
    let chars = s.chars();

    let mut cur = String::new();
    for (i, char) in chars.enumerate() {
        if (i + 1) % n == 0 {
            segments.push(cur);
            cur = String::new();
            continue;
        }
        cur.push(char);
    }
    segments.push(cur);

    segments
}
