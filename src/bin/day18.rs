use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops;

const DIRECTIONS: [Point; 6] = [
    Point(1, 0, 0),
    Point(-1, 0, 0),
    Point(0, 1, 0),
    Point(0, -1, 0),
    Point(0, 0, 1),
    Point(0, 0, -1),
];

fn main() {
    let input = parse_input();

    println!("Solution to part 1: {}", part1(&input));
    println!("Solution to part 2: {}", part2(&input));
}

fn part1(cubes: &HashSet<Point>) -> u32 {
    get_surface_area(&cubes)
}

fn part2(cubes: &HashSet<Point>) -> u32 {
    let (bottom_left, top_right) = get_bounding_box(cubes);

    let mut air_pockets: HashSet<Point> = HashSet::new();
    let mut total_visited: HashSet<Point> = HashSet::new();

    for x in bottom_left.0..=top_right.0 {
        for y in bottom_left.1..=top_right.1 {
            for z in bottom_left.2..=top_right.2 {
                let point = Point(x, y, z);
                if cubes.contains(&point) || total_visited.contains(&point) {
                    // Cube is not air or we already checked it in BFS
                    continue;
                }

                // Perform BFS
                let mut is_air_pocket = true;
                let mut visited: HashSet<Point> = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back(point);
                visited.insert(point);

                while let Some(v) = queue.pop_front() {
                    for direction in DIRECTIONS {
                        let p = v + direction;
                        if !in_bounding_box(p, bottom_left, top_right) {
                            is_air_pocket = false;
                            break;
                        }
                        if !visited.contains(&p) && !cubes.contains(&p) {
                            // Air block that hasn't been visited yet
                            visited.insert(p);
                            queue.push_back(p);
                        }
                    }
                }
                total_visited.extend(visited.iter());
                if is_air_pocket {
                    air_pockets.extend(visited.iter());
                }
            }
        }
    }
    get_surface_area(&cubes) - get_surface_area(&air_pockets)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32, i32);

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

fn get_surface_area(cubes: &HashSet<Point>) -> u32 {
    let mut surface_area = 0;

    for cube in cubes.iter() {
        for direction in DIRECTIONS {
            let pos = *cube + direction;
            if !cubes.contains(&pos) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn get_bounding_box(points: &HashSet<Point>) -> (Point, Point) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for point in points.iter() {
        min_x = min(point.0, min_x);
        min_y = min(point.1, min_x);
        min_z = min(point.2, min_x);
        max_x = max(point.0, max_x);
        max_y = max(point.1, max_y);
        max_z = max(point.2, max_z);
    }

    (Point(min_x, min_y, min_z), Point(max_x, max_y, max_z))
}

fn in_bounding_box(p: Point, bottom_left: Point, top_right: Point) -> bool {
    p.0 >= bottom_left.0
        && p.0 <= top_right.0
        && p.1 >= bottom_left.1
        && p.1 <= top_right.1
        && p.2 >= bottom_left.2
        && p.2 <= top_right.2
}

fn parse_input() -> HashSet<Point> {
    let mut cubes = HashSet::new();
    let file = File::open("inputs/day18.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let nums: Vec<i32> = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
            cubes.insert(Point(nums[0], nums[1], nums[2]));
        }
    }

    cubes
}
