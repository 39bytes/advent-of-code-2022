use std::cmp::max;
use std::{fs::File, io::Read};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let mut file = File::open("inputs/day8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file contents.");

    let grid: Vec<Vec<isize>> = contents
        .split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let (width, height) = (grid[0].len(), grid.len());

    let mut total_visible = 0;
    let mut max_scenic_score = 0;

    for i in 0..height {
        for j in 0..width {
            let mut visible = false;
            let mut scenic_score = 1;

            for direction in DIRECTIONS {
                let mut x = j as isize + direction.0;
                let mut y = i as isize + direction.1;

                let mut visible_in_dir = true;
                let mut view_dist = 0;
                while x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                    view_dist += 1;
                    if grid[y as usize][x as usize] >= grid[i][j] {
                        visible_in_dir = false;
                        break;
                    }

                    x += direction.0;
                    y += direction.1;
                }
                scenic_score *= view_dist;
                if visible_in_dir {
                    visible = true;
                }
            }

            max_scenic_score = max(max_scenic_score, scenic_score);
            if visible {
                total_visible += 1;
            }
        }
    }

    println!("Solution to part 1: {}", total_visible);
    println!("Solution to part 2: {}", max_scenic_score);
}
