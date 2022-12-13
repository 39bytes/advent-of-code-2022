use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part2();
}

fn part1() {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(sack) = line {
            let char_codes = sack.as_bytes();
            let half = char_codes.len() / 2;
            let first_sack: HashSet<u8> = HashSet::from_iter(char_codes[..half].iter().copied());
            let second_sack: HashSet<u8> = HashSet::from_iter(char_codes[half..].iter().copied());

            let char_code = first_intersect(&first_sack, &[&second_sack]);

            sum += get_char_priority(char_code) as i32;
        }
    }

    println!("Total: {}", sum);
}

fn part2() {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut lines = reader.lines().map(|l| l.unwrap());
    loop {
        if let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next())
        {
            let first_sack: HashSet<u8> = HashSet::from_iter(line1.as_bytes().iter().copied());
            let second_sack: HashSet<u8> = HashSet::from_iter(line2.as_bytes().iter().copied());
            let third_sack: HashSet<u8> = HashSet::from_iter(line3.as_bytes().iter().copied());

            let char_code = first_intersect(&first_sack, &[&second_sack, &third_sack]);

            sum += get_char_priority(char_code) as i32;
        } else {
            break;
        }
    }
    println!("Total: {}", sum);
}

fn first_intersect(set: &HashSet<u8>, others: &[&HashSet<u8>]) -> u8 {
    *set.iter()
        .filter(|b| others.iter().all(|s| s.contains(b)))
        .next()
        .expect("No intersection found.")
}

fn get_char_priority(char_code: u8) -> u8 {
    match char_code {
        65..=90 => char_code - 38,
        97..=122 => char_code - 96,
        _ => panic!("Invalid character"),
    }
}
