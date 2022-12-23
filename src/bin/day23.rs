use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let elves = parse_input("inputs/day23.txt");

    println!("Solution to part 1: {}", part1(elves.clone()));
    println!("Solution to part 2: {}", part2(elves.clone()));
}

fn part1(initial_state: HashSet<Elf>) -> u32 {
    let mut rounds = 0;
    let mut elves = initial_state;

    let mut directions = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    while rounds < 10 {
        let mut proposed: HashMap<Point, Vec<Elf>> = HashMap::new();
        for elf in elves.iter() {
            let n = elves.contains(&(elf.0, elf.1 - 1));
            let ne = elves.contains(&(elf.0 + 1, elf.1 - 1));
            let nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
            let s = elves.contains(&(elf.0, elf.1 + 1));
            let se = elves.contains(&(elf.0 + 1, elf.1 + 1));
            let sw = elves.contains(&(elf.0 - 1, elf.1 + 1));
            let e = elves.contains(&(elf.0 + 1, elf.1));
            let w = elves.contains(&(elf.0 - 1, elf.1));

            // Check all adjacent
            if [n, ne, nw, s, se, sw, e, w].iter().all(|x| !x) {
                proposed.entry(*elf).or_default().push(*elf);
                continue;
            }

            let proposed_dir = {
                let mut proposed_dir = None;
                for dir in &directions {
                    let to_check = match dir {
                        Direction::North => [n, ne, nw],
                        Direction::South => [s, se, sw],
                        Direction::West => [w, nw, sw],
                        Direction::East => [e, ne, se],
                    };

                    if to_check.iter().all(|x| !x) {
                        proposed_dir = Some(dir);
                        break;
                    }
                }
                proposed_dir
            };

            if let Some(dir) = proposed_dir {
                let new_pos = match dir {
                    Direction::North => (elf.0, elf.1 - 1),
                    Direction::South => (elf.0, elf.1 + 1),
                    Direction::West => (elf.0 - 1, elf.1),
                    Direction::East => (elf.0 + 1, elf.1),
                };
                proposed.entry(new_pos).or_default().push(*elf);
            } else {
                proposed.entry(*elf).or_default().push(*elf);
            }
        }

        let mut new_elves = HashSet::new();
        for (pos, proposed_elves) in proposed {
            if proposed_elves.len() == 1 {
                new_elves.insert(pos);
            } else {
                for elf in proposed_elves.iter() {
                    new_elves.insert(*elf);
                }
            }
        }
        elves = new_elves;

        let d = directions.pop_front().unwrap();
        directions.push_back(d);
        rounds += 1;
    }

    get_empty_tiles(&elves)
}

fn part2(initial_state: HashSet<Elf>) -> u32 {
    let mut rounds = 0;
    let mut elves = initial_state;

    let mut directions = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    loop {
        let mut proposed: HashMap<Point, Vec<Elf>> = HashMap::new();
        let mut moved_this_round = false;
        for elf in elves.iter() {
            let n = elves.contains(&(elf.0, elf.1 - 1));
            let ne = elves.contains(&(elf.0 + 1, elf.1 - 1));
            let nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
            let s = elves.contains(&(elf.0, elf.1 + 1));
            let se = elves.contains(&(elf.0 + 1, elf.1 + 1));
            let sw = elves.contains(&(elf.0 - 1, elf.1 + 1));
            let e = elves.contains(&(elf.0 + 1, elf.1));
            let w = elves.contains(&(elf.0 - 1, elf.1));

            // Check all adjacent
            if [n, ne, nw, s, se, sw, e, w].iter().all(|x| !x) {
                proposed.entry(*elf).or_default().push(*elf);
                continue;
            }
            moved_this_round = true;

            let proposed_dir = {
                let mut proposed_dir = None;
                for dir in &directions {
                    let to_check = match dir {
                        Direction::North => [n, ne, nw],
                        Direction::South => [s, se, sw],
                        Direction::West => [w, nw, sw],
                        Direction::East => [e, ne, se],
                    };

                    if to_check.iter().all(|x| !x) {
                        proposed_dir = Some(dir);
                        break;
                    }
                }
                proposed_dir
            };

            if let Some(dir) = proposed_dir {
                let new_pos = match dir {
                    Direction::North => (elf.0, elf.1 - 1),
                    Direction::South => (elf.0, elf.1 + 1),
                    Direction::West => (elf.0 - 1, elf.1),
                    Direction::East => (elf.0 + 1, elf.1),
                };
                proposed.entry(new_pos).or_default().push(*elf);
            } else {
                proposed.entry(*elf).or_default().push(*elf);
            }
        }

        if !moved_this_round {
            return rounds + 1;
        }
        let mut new_elves = HashSet::new();
        for (pos, proposed_elves) in proposed {
            if proposed_elves.len() == 1 {
                new_elves.insert(pos);
            } else {
                for elf in proposed_elves.iter() {
                    new_elves.insert(*elf);
                }
            }
        }
        elves = new_elves;

        let d = directions.pop_front().unwrap();
        directions.push_back(d);
        rounds += 1;
    }
}

fn get_bounding_box(elves: &HashSet<Elf>) -> (Point, Point) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for point in elves {
        min_x = min(point.0, min_x);
        min_y = min(point.1, min_y);
        max_x = max(point.0, max_x);
        max_y = max(point.1, max_y);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn get_empty_tiles(elves: &HashSet<Elf>) -> u32 {
    let (top_left, bottom_right) = get_bounding_box(&elves);

    let mut num_empty = 0;
    for x in top_left.0..=bottom_right.0 {
        for y in top_left.1..=bottom_right.1 {
            if !elves.contains(&(x, y)) {
                num_empty += 1;
            }
        }
    }

    num_empty
}

fn parse_input(path: &str) -> HashSet<Elf> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut elves = HashSet::new();

    for (y, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }
    }

    elves
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Elf = (isize, isize);
type Point = (isize, isize);
