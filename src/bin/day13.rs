use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Solution to part 1: {}", part1());
    println!("Solution to part 2: {}", part2());
}

fn part1() -> u32 {
    let mut packets = VecDeque::from(parse_input());

    let mut i = 1;
    let mut index_sum = 0;
    while !packets.is_empty() {
        let left = packets.pop_front().unwrap();
        let right = packets.pop_front().unwrap();
        if left.cmp(&right) == Ordering::Less {
            index_sum += i;
        }
        i += 1;
    }
    index_sum
}

fn part2() -> usize {
    let mut packets = parse_input();

    let first = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let second = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(first.clone());
    packets.push(second.clone());
    packets.sort();

    let first_index = packets.iter().position(|p| p == &first).unwrap() + 1;
    let second_index = packets.iter().position(|p| p == &second).unwrap() + 1;

    first_index * second_index
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Packet::List(left), Packet::List(right)) = (self, other) {
            for (a, b) in left.iter().zip(right.iter()) {
                let comparison = match (a, b) {
                    (Packet::List(_), Packet::List(_)) => a.cmp(b),
                    (Packet::List(_), Packet::Int(right)) => {
                        let right = Packet::List(vec![Packet::Int(*right)]);
                        a.cmp(&right)
                    }
                    (Packet::Int(left), Packet::List(_)) => {
                        let left = Packet::List(vec![Packet::Int(*left)]);
                        left.cmp(&b)
                    }
                    (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
                };
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
            return left.len().cmp(&right.len());
        }
        Ordering::Equal
    }
}

fn parse_input() -> Vec<Packet> {
    let file = File::open("inputs/day13.txt").unwrap();
    let reader = BufReader::new(file);

    let mut packets = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                packets.push(parse_packet(&line));
            }
        }
    }
    packets
}

fn parse_packet(packet_str: &str) -> Packet {
    parse_packet_recursive(packet_str, 0).0
}

fn parse_packet_recursive(packet_str: &str, start_index: usize) -> (Packet, usize) {
    let mut packet_contents: Vec<Packet> = Vec::new();
    let mut cur_num = String::new();

    let mut i = start_index;
    let chars: Vec<char> = packet_str.chars().collect();

    while i < chars.len() {
        match chars[i] {
            '[' => {
                let (packet, end_index) = parse_packet_recursive(&packet_str, i + 1);
                packet_contents.push(packet);
                i = end_index;
            }
            ']' => {
                if !cur_num.is_empty() {
                    packet_contents.push(Packet::Int(cur_num.parse().unwrap()));
                }
                return (Packet::List(packet_contents), i);
            }
            ',' => {
                if !cur_num.is_empty() {
                    packet_contents.push(Packet::Int(cur_num.parse().unwrap()));
                }
                cur_num = String::new();
            }
            digit => cur_num.push(digit),
        }
        i += 1;
    }
    (Packet::List(packet_contents), packet_str.len() - 1)
}
