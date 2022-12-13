use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, path,
};

fn main() {
    let buffer = fs::read_to_string("inputs/day12.txt").unwrap();

    let grid: Vec<Vec<char>> = buffer
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect();

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut start_pos: (isize, isize) = (0, 0);
    let mut dest: (isize, isize) = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let char = grid[i][j];
            if char == 'S' {
                start_pos = (i as isize, j as isize);
            } else if char == 'E' {
                dest = (i as isize, j as isize);
            }
        }
    }

    println!("Solution to part 1: {}", bfs(&grid, start_pos, dest));
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut start_positions = vec![];
    let mut dest: (isize, isize) = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let char = grid[i][j];
            if char == 'S' || char == 'a' {
                start_positions.push((i as isize, j as isize));
            } else if char == 'E' {
                dest = (i as isize, j as isize);
            }
        }
    }

    let min = start_positions
        .iter()
        .map(|pos| bfs(&grid, *pos, dest))
        .min()
        .unwrap();
    println!("Solution to part 2: {}", min);
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn bfs(grid: &Vec<Vec<char>>, start: (isize, isize), dest: (isize, isize)) -> usize {
    let mut queue = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut prev: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        for dir in DIRECTIONS {
            let mut adjacent = (pos.0 + dir.0, pos.1 + dir.1);
            if !visited.contains(&adjacent) && can_move(&grid, pos, adjacent) {
                visited.insert(adjacent);
                prev.insert(adjacent, pos);
                queue.push_back(adjacent);

                if adjacent == dest {
                    let mut path_len = 0;
                    while let Some(v) = prev.get(&adjacent) {
                        path_len += 1;
                        adjacent = *v;
                    }
                    return path_len;
                }
            }
        }
    }
    usize::MAX
}

fn can_move(grid: &Vec<Vec<char>>, from: (isize, isize), to: (isize, isize)) -> bool {
    if to.0 < 0 || to.0 >= grid.len() as isize || to.1 < 0 || to.1 >= grid[0].len() as isize {
        return false;
    }

    let mut char = grid[from.0 as usize][from.1 as usize];
    char = if char == 'S' {
        'a'
    } else if char == 'E' {
        'z'
    } else {
        char
    };
    let height = char as i16;
    let adj_char = grid[to.0 as usize][to.1 as usize];
    let adj_height = adj_char as i16;

    if adj_height - height > 1 {
        return false;
    }
    true
}
