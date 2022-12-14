use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

fn main() {
    println!("Solution to part 1: {}", part1());
    println!("Solution to part 2: {}", part2());
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn from_str(s: &str) -> Point {
        let coords: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
        Point(coords[0], coords[1])
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn part1() -> u32 {
    let mut filled_positions = parse_input();

    let mut num_sand = 0;
    let mut done = false;

    while !done {
        let mut sand_pos = Point(500, 0);
        loop {
            if sand_pos.1 >= 200 {
                done = true;
                break;
            }
            let down = sand_pos + Point(0, 1);
            let down_left = sand_pos + Point(-1, 1);
            let down_right = sand_pos + Point(1, 1);

            if !filled_positions.contains(&down) {
                sand_pos = down;
            } else if !filled_positions.contains(&down_left) {
                sand_pos = down_left;
            } else if !filled_positions.contains(&down_right) {
                sand_pos = down_right;
            } else {
                num_sand += 1;
                filled_positions.insert(sand_pos);
                break;
            }
        }
    }
    num_sand
}

fn part2() -> u32 {
    let mut filled_positions = parse_input();
    let floor_y = filled_positions.iter().map(|pos| pos.1).max().unwrap() + 2;

    let mut num_sand = 0;

    loop {
        let mut sand_pos = Point(500, 0);
        if filled_positions.contains(&sand_pos) {
            break;
        }
        loop {
            let down = sand_pos + Point(0, 1);
            let down_left = sand_pos + Point(-1, 1);
            let down_right = sand_pos + Point(1, 1);

            if !(filled_positions.contains(&down) || down.1 >= floor_y) {
                sand_pos = down;
            } else if !(filled_positions.contains(&down_left) || down_left.1 >= floor_y) {
                sand_pos = down_left;
            } else if !(filled_positions.contains(&down_right) || down_right.1 >= floor_y) {
                sand_pos = down_right;
            } else {
                num_sand += 1;
                filled_positions.insert(sand_pos);
                break;
            }
        }
    }
    num_sand
}

fn parse_input() -> HashSet<Point> {
    let file = File::open("inputs/day14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut filled_positions: HashSet<Point> = HashSet::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let path_points = parse_path_points(line);
            filled_positions.extend(path_points.iter());
        }
    }
    filled_positions
}

fn parse_path_points(path_str: String) -> Vec<Point> {
    let path: Vec<&str> = path_str.split(" -> ").collect();
    let mut points: Vec<Point> = Vec::new();

    let mut cur_point = Point::from_str(path[0]);

    for point in path.iter().skip(1) {
        let point = Point::from_str(point);

        let (start_point, end_point) = if cur_point.0 < point.0 || cur_point.1 < point.1 {
            (cur_point, point)
        } else {
            (point, cur_point)
        };

        for x in start_point.0..=end_point.0 {
            for y in start_point.1..=end_point.1 {
                points.push(Point(x, y));
            }
        }
        cur_point = point;
    }

    points
}
