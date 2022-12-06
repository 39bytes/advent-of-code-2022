use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day6.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error reading file to string.");

    let mut packet = 0;
    let mut message = 0;
    for (i, _) in data.char_indices() {
        if is_marker(&data[i..i + 4]) {
            if is_marker(&data[i..i + 14]) {
                if message == 0 {
                    message = i + 14;
                    break;
                }
            }
            if packet == 0 {
                packet = i + 4
            };
        }
    }
    println!("Solution to part 1: {}", packet);
    println!("Solution to part 2: {}", message);
}

fn is_marker(s: &str) -> bool {
    let seq = s.chars().collect::<HashSet<char>>();
    seq.len() == s.len()
}
