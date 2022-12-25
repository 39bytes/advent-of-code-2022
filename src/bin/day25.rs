use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = parse_input("inputs/day25.txt");
    println!("Solution to part 1: {}", part1(&input));
}

fn part1(input: &Vec<String>) -> String {
    let dec = input.iter().map(|s| snafu_to_decimal(s)).sum();
    decimal_to_snafu(dec)
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut dec = 0;
    for (i, c) in snafu.chars().enumerate() {
        let power = (snafu.len() - i - 1) as u32;
        dec += snafu_digit_to_dec(c) * (5_i64.pow(power));
    }
    return dec;
}

fn decimal_to_snafu(dec: i64) -> String {
    let mut digits: Vec<char> = Vec::new();
    let mut num = dec;

    while num > 0 {
        num += 2;
        digits.push(match num % 5 {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        });
        num /= 5;
    }

    digits.iter().rev().collect()
}

fn snafu_digit_to_dec(digit: char) -> i64 {
    match digit {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Invalid digit."),
    }
}

fn dec_digit_to_snafu(digit: i64) -> char {
    match digit {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("Invalid digit."),
    }
}

fn parse_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|x| x.ok())
        .filter(|s| !s.is_empty())
        .collect()
}
