use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut total = 0;

    for line in reader.lines() {
        if let Ok(val) = line {
            if &val == "" {
                totals.push(total);
                total = 0;
            } else {
                let calories = val.parse::<i32>().expect("Value was not a number.");
                total += calories;
            }
        }
    }

    totals.sort();

    let len = totals.len();
    let top_3_sum = totals[len - 1] + totals[len - 2] + totals[len - 3];

    println!("Part 1: {}", totals[len - 1]);
    println!("Part 2: {}", top_3_sum);
}
