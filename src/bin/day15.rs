use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

fn main() {
    println!("Solution to part 1: {}", part1());
    println!("Solution to part 2: {}", part2());
}

fn part1() -> i32 {
    let (sensors, beacons) = parse_input();

    let mut num_covered = 0;
    let mut x = -2_000_000;
    while x <= 5_000_000 {
        let p = Point::new(x, 2_000_000);
        for sensor in &sensors {
            if sensor.in_radius(p) && !beacons.contains(&p) {
                let skip =
                    sensor.position.x - x - (sensor.position.y - 2_000_000).abs() + sensor.radius;
                num_covered += skip + 1;
                x += skip;
                break;
            }
        }
        x += 1;
    }
    num_covered - 1
}

fn part2() -> i64 {
    let (sensors, _) = parse_input();

    let mut x = 0;
    let mut y = 0;
    while y <= 4_000_000 {
        while x <= 4_000_000 {
            let p = Point::new(x, y);
            let mut found = true;
            for sensor in &sensors {
                if sensor.in_radius(p) {
                    let skip =
                        sensor.position.x - x - (sensor.position.y - y).abs() + sensor.radius + 1;
                    x += skip;
                    found = false;
                    break;
                }
            }
            if found {
                return p.x as i64 * 4_000_000 + p.y as i64;
            }
        }
        x = 0;
        y += 1;
    }
    -1
}

fn parse_input() -> (Vec<Sensor>, Vec<Point>) {
    let file = File::open("inputs/day15.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let (sensor, beacon) = parse_sensor_line(line);
            sensors.push(sensor);
            beacons.push(beacon);
        }
    }
    (sensors, beacons)
}

fn parse_sensor_line(line: String) -> (Sensor, Point) {
    let parts: Vec<&str> = line.split(": ").collect();
    let sensor_pos = parts[0].strip_prefix("Sensor at ").unwrap();
    let beacon_pos = parts[1].strip_prefix("closest beacon is at ").unwrap();

    let sensor_pos = Point::from_str(sensor_pos);
    let beacon_pos = Point::from_str(beacon_pos);

    let sensor_radius = sensor_pos.manhattan_dist(beacon_pos);

    return (
        Sensor {
            position: sensor_pos,
            radius: sensor_radius,
        },
        beacon_pos,
    );
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn from_str(s: &str) -> Point {
        let coords: Vec<&str> = s.split(", ").collect();
        Point {
            x: coords[0].strip_prefix("x=").unwrap().parse().unwrap(),
            y: coords[1].strip_prefix("y=").unwrap().parse().unwrap(),
        }
    }

    fn manhattan_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    radius: i32,
}

impl Sensor {
    fn in_radius(&self, p: Point) -> bool {
        self.position.manhattan_dist(p) <= self.radius
    }
}
