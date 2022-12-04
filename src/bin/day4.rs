use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut complete_overlaps = 0;
    let mut overlaps = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let assignments: Vec<u32> = line
                .split(",")
                .map(|x| {
                    x.split("-")
                        .map(|n| n.parse::<u32>().expect("String was not an integer."))
                })
                .flatten()
                .collect();

            let pair1 = (assignments[0], assignments[1]);
            let pair2 = (assignments[2], assignments[3]);
            if complete_overlap(pair1, pair2) {
                complete_overlaps += 1;
            }
            if overlap(pair1, pair2) {
                overlaps += 1;
            }
        }
    }
    println!("Solution to part 1: {}", complete_overlaps);
    println!("Solution to part 2: {}", overlaps);
}

fn overlap(pair1: (u32, u32), pair2: (u32, u32)) -> bool {
    pair1.1 <= pair2.1 && pair1.1 >= pair2.0 || pair2.1 <= pair1.1 && pair2.1 >= pair1.0
}

fn complete_overlap(pair1: (u32, u32), pair2: (u32, u32)) -> bool {
    pair1.0 >= pair2.0 && pair1.1 <= pair2.1 || pair2.0 >= pair1.0 && pair2.1 <= pair1.1
}
