use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

fn main() {
    println!(
        "Solution to part 1: {}",
        get_num_unique_tail_positions(&mut [Position::origin(); 2])
    );
    println!(
        "Solution to part 2: {}",
        get_num_unique_tail_positions(&mut [Position::origin(); 10])
    );
}

fn get_num_unique_tail_positions(knots: &mut [Position]) -> usize {
    let file = File::open("inputs/day9.txt").unwrap();
    let reader = BufReader::new(file);

    let mut unique_positions: HashSet<Position> = HashSet::new();

    unique_positions.insert(*knots.last().unwrap());

    for line in reader.lines() {
        if let Ok(line) = line {
            let (direction, steps) = parse_move_line(line);
            for _ in 0..steps {
                knots[0] = knots[0] + direction;
                for i in 1..knots.len() {
                    let diff = knots[i - 1] - knots[i];
                    if (diff.x * diff.x + diff.y * diff.y) > 2 {
                        knots[i] = knots[i] + Position::new(normalize(diff.x), normalize(diff.y))
                    }
                }
                unique_positions.insert(*knots.last().unwrap());
            }
        }
    }
    unique_positions.len()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn origin() -> Position {
        Position { x: 0, y: 0 }
    }
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    fn from_name(name: &str) -> Position {
        match name {
            "L" => Position { x: -1, y: 0 },
            "R" => Position { x: 1, y: 0 },
            "U" => Position { x: 0, y: 1 },
            "D" => Position { x: 0, y: -1 },
            _ => panic!("Invalid direction name!"),
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn parse_move_line(s: String) -> (Position, i32) {
    let vals: Vec<&str> = s.split_whitespace().collect();
    let direction = Position::from_name(vals[0]);
    let steps: i32 = vals[1].parse().expect("Failed to parse number of steps");

    (direction, steps)
}

fn normalize(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}
