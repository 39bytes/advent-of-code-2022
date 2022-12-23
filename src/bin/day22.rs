use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (map, moves) = parse_input("inputs/day22.txt");
    let moves_vec = parse_moves(&moves);

    println!("Solution to part 1: {}", part1(&map, &moves_vec));
    println!("Solution to part 2: {}", part2(&map, &moves_vec));
}

const DIRECTIONS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

fn part1(map: &Vec<Vec<char>>, moves: &Vec<&str>) -> i32 {
    let start_x = get_row_start_x(map, 0) as i32;

    let mut pos = Point(start_x, 0);
    let mut dir = 0;

    for mov in moves.iter() {
        if let Ok(steps) = mov.parse::<u32>() {
            for _ in 0..steps {
                let new_pos = pos.wrap_add(map, &DIRECTIONS[dir]);
                if map[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                    pos = new_pos;
                } else {
                    break;
                }
            }
        } else {
            match *mov {
                "L" => dir = (dir as i32 - 1).rem_euclid(4) as usize,
                "R" => dir = (dir as i32 + 1).rem_euclid(4) as usize,
                _ => panic!("Invalid turn"),
            }
        }
    }
    (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + dir as i32
}

fn part2(map: &Vec<Vec<char>>, moves: &Vec<&str>) -> i32 {
    let start_x = get_row_start_x(map, 0) as i32;

    let mut pos = Point(start_x, 0);
    let mut dir = 0;

    for mov in moves.iter() {
        if let Ok(steps) = mov.parse::<u32>() {
            for _ in 0..steps {
                let (new_pos, new_dir) = pos.wrap_face_add(map, dir);
                if map[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                    pos = new_pos;
                    dir = new_dir;
                } else {
                    break;
                }
            }
        } else {
            match *mov {
                "L" => dir = (dir as i32 - 1).rem_euclid(4) as usize,
                "R" => dir = (dir as i32 + 1).rem_euclid(4) as usize,
                _ => panic!("Invalid turn"),
            }
        }
    }
    (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + dir as i32
}

fn get_row_start_x(map: &Vec<Vec<char>>, row_num: usize) -> usize {
    let row = &map[row_num];
    let mut x = 0;

    while x < row.len() && row[x] == ' ' {
        x += 1;
    }

    x
}

fn get_col_start_y(map: &Vec<Vec<char>>, col_num: usize) -> usize {
    let mut y = 0;

    for row in map.iter() {
        if row[col_num] != ' ' {
            break;
        }
        y += 1;
    }

    y
}

fn get_row_width(map: &Vec<Vec<char>>, row_num: usize) -> usize {
    let row = &map[row_num];
    let spaces: Vec<&char> = row.iter().filter(|c| **c == ' ').collect();
    row.len() - spaces.len()
}

fn get_col_height(map: &Vec<Vec<char>>, col_num: usize) -> usize {
    let mut height = 0;
    for row in map.iter() {
        if row[col_num] != ' ' {
            height += 1;
        }
    }
    height
}

fn parse_input(path: &str) -> (Vec<Vec<char>>, String) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let map_width = lines.iter().max_by_key(|l| l.len()).unwrap().len();

    for line in lines.iter().take(lines.len() - 2) {
        let mut row: Vec<char> = line.chars().collect();

        // Pad map with spaces to make a rectangular map
        row.extend(vec![' '; map_width - row.len()].iter());
        map.push(row);
    }

    (map, lines.last().unwrap().to_string())
}

fn parse_moves(moves: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in moves.match_indices(|c: char| !(c.is_numeric())) {
        if last != index {
            result.push(&moves[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < moves.len() {
        result.push(&moves[last..]);
    }

    result
}

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn wrap_add(&self, map: &Vec<Vec<char>>, dir: &Point) -> Point {
        let start_x = get_row_start_x(map, self.1 as usize) as i32;
        let start_y = get_col_start_y(map, self.0 as usize) as i32;

        let width = get_row_width(map, self.1 as usize) as i32;
        let height = get_col_height(map, self.0 as usize) as i32;

        let new_x = (self.0 + dir.0 - start_x).rem_euclid(width) + start_x;
        let new_y = (self.1 + dir.1 - start_y).rem_euclid(height) + start_y;
        Point(new_x, new_y)
    }

    fn wrap_face_add(&self, map: &Vec<Vec<char>>, dir: usize) -> (Point, usize) {
        // 0 - Right
        // 1 - Down
        // 2 - Left
        // 3 - Up
        let d = &DIRECTIONS[dir];
        let new_pos = Point(self.0 + d.0, self.1 + d.1);

        // Check if new position is inside the 2d map
        if new_pos.0 >= 0
            && new_pos.0 < map[0].len() as i32
            && new_pos.1 >= 0
            && new_pos.1 < map.len() as i32
            && map[new_pos.1 as usize][new_pos.0 as usize] != ' '
        {
            return (new_pos, dir);
        }

        // Move through cube edges
        match self {
            // Face 1
            Point(50..=99, 0..=49) => match dir {
                2 => {
                    let y_offset = 49 - self.1;
                    (Point(0, 100 + y_offset), 0)
                }
                3 => {
                    let y_offset = self.0 - 50;
                    (Point(0, 150 + y_offset), 0)
                }
                _ => unreachable!(),
            },
            // Face 2
            Point(100..=149, 0..=49) => match dir {
                0 => {
                    let y_offset = 49 - self.1;
                    (Point(99, 100 + y_offset), 2)
                }
                1 => {
                    let y_offset = self.0 - 100;
                    (Point(99, 50 + y_offset), 2)
                }
                3 => (Point(self.0 - 100, 199), 3),
                _ => unreachable!(),
            },
            // Face 3
            Point(50..=99, 50..=99) => match dir {
                0 => {
                    let x_offset = self.1 - 50;
                    (Point(100 + x_offset, 49), 3)
                }
                2 => {
                    let x_offset = self.1 - 50;
                    (Point(x_offset, 100), 1)
                }
                _ => unreachable!(),
            },
            // Face 4
            Point(0..=49, 100..=149) => match dir {
                2 => {
                    let y_offset = 49 - (self.1 - 100);
                    (Point(50, y_offset), 0)
                }
                3 => {
                    let y_offset = self.0;
                    (Point(50, 50 + y_offset), 0)
                }
                _ => unreachable!(),
            },
            // Face 5
            Point(50..=99, 100..=149) => match dir {
                0 => {
                    let y_offset = 49 - (self.1 - 100);
                    (Point(149, y_offset), 2)
                }
                1 => {
                    let y_offset = self.0 - 50;
                    (Point(49, 150 + y_offset), 2)
                }
                _ => unreachable!(),
            },
            // Face 6
            Point(0..=49, 150..=199) => match dir {
                0 => {
                    let x_offset = self.1 - 150;
                    (Point(50 + x_offset, 149), 3)
                }
                1 => {
                    let x_offset = self.0;
                    (Point(100 + x_offset, 0), 1)
                }
                2 => {
                    let x_offset = self.1 - 150;
                    (Point(50 + x_offset, 0), 1)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
