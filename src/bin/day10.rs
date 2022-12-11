use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day10.txt").unwrap();
    let reader = BufReader::new(file);

    const INTERESTING_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

    let mut signal_strength_sum = 0;

    let mut x = 1;
    let mut cycle = 0;

    let mut output = String::new();
    let mut crt_col = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let instruction: Vec<&str> = line.split_whitespace().collect();
            let required_cycles = get_num_required_cycles(instruction[0]);
            for _ in 0..required_cycles {
                if crt_col == x - 1 || crt_col == x || crt_col == x + 1 {
                    output.push('#');
                } else {
                    output.push('.');
                }

                crt_col += 1;
                if crt_col % 40 == 0 {
                    crt_col = 0;
                    output.push('\n');
                }

                cycle += 1;
                if INTERESTING_CYCLES.contains(&cycle) {
                    signal_strength_sum += cycle * x;
                }
            }
            match instruction[0] {
                "noop" => {}
                "addx" => {
                    let val: i32 = instruction[1]
                        .parse()
                        .expect("Argument to addx was not an integer.");
                    x += val;
                }
                _ => panic!("Invalid instruction."),
            }
        }
    }
    println!("Solution to part 1: {}", signal_strength_sum);
    println!("Solution to part 2: \n{output}");
}

fn get_num_required_cycles(instruction: &str) -> u32 {
    match instruction {
        "noop" => 1,
        "addx" => 2,
        _ => panic!("Invalid instruction."),
    }
}
