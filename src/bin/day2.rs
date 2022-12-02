use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part2();
}

fn part1() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        if let Ok(shapes) = line {
            let matchup: Vec<&str> = shapes.split(" ").collect();

            match matchup[1] {
                "X" => {
                    total += 1;
                    match matchup[0] {
                        "A" => total += 3,
                        "C" => total += 6,
                        _ => {}
                    }
                }
                "Y" => {
                    total += 2;
                    match matchup[0] {
                        "B" => total += 3,
                        "A" => total += 6,
                        _ => {}
                    }
                }
                "Z" => {
                    total += 3;
                    match matchup[0] {
                        "C" => total += 3,
                        "B" => total += 6,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    println!("Total score: {}", total);
}

fn part2() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let lose_matchup = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    let win_matchup = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let pt_vals = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let mut total = 0;
    for line in reader.lines() {
        if let Ok(shapes) = line {
            let matchup: Vec<&str> = shapes.split(" ").collect();

            let shape = match matchup[1] {
                "X" => lose_matchup.get(matchup[0]).unwrap(),
                "Y" => {
                    total += 3;
                    matchup[0]
                }
                "Z" => {
                    total += 6;
                    win_matchup.get(matchup[0]).unwrap()
                }
                _ => panic!("Invalid shape."),
            };
            total += pt_vals.get(shape).unwrap();
        }
    }
    println!("Total score: {}", total);
}
