use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops;

fn main() {
    let (initial_state, map, start, goal) = parse_input("inputs/day24.txt");

    let mut prev_state = initial_state;
    let mut blizzard_states: Vec<Vec<Point>> = vec![prev_state.iter().map(|b| b.pos).collect()];

    for _ in 0..1000 {
        let next_state = next_blizzard_state(&prev_state, &map);
        blizzard_states.push(next_state.iter().map(|b| b.pos).collect());
        prev_state = next_state;
    }

    println!(
        "Solution to part 1: {}",
        part1(&blizzard_states, &map, start, goal)
    );
    println!(
        "Solution to part 2: {}",
        part2(&blizzard_states, &map, start, goal)
    );
}

fn part1(
    blizzard_states: &Vec<Vec<Point>>,
    map: &Vec<Vec<char>>,
    start: Point,
    goal: Point,
) -> usize {
    return bfs(blizzard_states, map, start, goal, 0);
}

fn part2(
    blizzard_states: &Vec<Vec<Point>>,
    map: &Vec<Vec<char>>,
    start: Point,
    goal: Point,
) -> usize {
    let first = bfs(blizzard_states, map, start, goal, 0);
    let second = bfs(blizzard_states, map, goal, start, first);
    let third = bfs(blizzard_states, map, start, goal, first + second);
    first + second + third
}

fn bfs(
    blizzard_states: &Vec<Vec<Point>>,
    map: &Vec<Vec<char>>,
    start: Point,
    goal: Point,
    start_time: usize,
) -> usize {
    let mut seen: HashSet<(Point, usize)> = HashSet::new();
    seen.insert((start, start_time));
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    queue.push_back((start, start_time));

    while let Some((pos, steps_taken)) = queue.pop_front() {
        if pos == goal {
            return steps_taken - start_time;
        }

        for dir in DIRECTIONS {
            let new_pos = pos + dir;
            if new_pos.1 >= 0
                && new_pos.1 < map.len() as isize
                && map[new_pos.1 as usize][new_pos.0 as usize] != '#'
                && !blizzard_states[steps_taken + 1].contains(&new_pos)
                && !seen.contains(&(new_pos, steps_taken + 1))
            {
                seen.insert((new_pos, steps_taken + 1));
                queue.push_back((new_pos, steps_taken + 1));
            }
        }
    }

    usize::MAX
}

fn next_blizzard_state(state: &Vec<Blizzard>, map: &Vec<Vec<char>>) -> Vec<Blizzard> {
    let mut next = Vec::new();
    let right_wall_x = (map[0].len() - 1) as isize;
    let bottom_wall_y = (map.len() - 1) as isize;

    for blizzard in state.iter() {
        let mut next_pos = blizzard.pos + blizzard.dir;

        if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            if next_pos.0 == 0 {
                next_pos = Point(right_wall_x - 1, next_pos.1);
            } else if next_pos.0 == right_wall_x {
                next_pos = Point(1, next_pos.1);
            } else if next_pos.1 == 0 {
                next_pos = Point(next_pos.0, bottom_wall_y - 1);
            } else if next_pos.1 == bottom_wall_y {
                next_pos = Point(next_pos.0, 1);
            } else {
                unreachable!()
            }
        }

        next.push(Blizzard {
            pos: next_pos,
            dir: blizzard.dir,
        });
    }

    next
}

fn parse_input(path: &str) -> (Vec<Blizzard>, Vec<Vec<char>>, Point, Point) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut blizzards = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    let lines: Vec<String> = reader.lines().filter_map(|x| x.ok()).collect();

    for (y, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut row: Vec<char> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == '#' || c == '.' {
                row.push(c);
                continue;
            }

            let dir = match c {
                '>' => DIRECTIONS[0],
                '<' => DIRECTIONS[1],
                '^' => DIRECTIONS[2],
                'v' => DIRECTIONS[3],
                _ => panic!("Invalid character."),
            };

            blizzards.push(Blizzard {
                pos: Point(x as isize, y as isize),
                dir,
            });
            row.push('.');
        }
        map.push(row);
    }

    let start_x = map[0].iter().position(|c| *c == '.').unwrap() as isize;
    let goal_x = map[map.len() - 1].iter().position(|c| *c == '.').unwrap() as isize;
    let goal_y = (map.len() - 1) as isize;

    (blizzards, map, Point(start_x, 0), Point(goal_x, goal_y))
}

const DIRECTIONS: [Point; 5] = [
    Point(1, 0),
    Point(-1, 0),
    Point(0, -1),
    Point(0, 1),
    Point(0, 0),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    pos: Point,
    dir: Point,
}
