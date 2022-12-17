use std::fs;
use std::time::Instant;

const GRID_WIDTH: usize = 7;

fn main() {
    let shapes: Vec<Vec<Vec<char>>> = vec![
        vec![vec!['#', '#', '#', '#']],
        vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ],
        vec![
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ],
        vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']],
        vec![vec!['#', '#'], vec!['#', '#']],
    ];
    let moves = fs::read_to_string("inputs/day17.txt").unwrap();

    println!("Solution to part 1: {}", solve(&shapes, moves.trim(), 2022));
    println!(
        "Solution to part 2: {}",
        solve(&shapes, moves.trim(), 1_000_000_000_000)
    );
}

fn solve(shapes: &Vec<Vec<Vec<char>>>, moves: &str, n: usize) -> usize {
    let mut board = Board::new();
    let moves: Vec<char> = moves.chars().collect();
    let mut move_index: usize = 0;
    let mut num_landed = 0;
    let mut cur_piece = Piece {
        x: 2,
        y: 3,
        shape: &shapes[num_landed],
    };

    let mut remaining_pieces = n;
    let mut start_height = 0; // The height before the cycle begins
    let mut num_landed_before_cycle = 0; // The height before the cycle begins
    let mut cycle_height = 0;
    let mut total_height = 0;

    while remaining_pieces > 0 {
        match moves[move_index] {
            '>' => {
                let right = (1, 0);

                if board.can_move(right, &cur_piece) {
                    cur_piece.move_piece(right);
                }
            }
            '<' => {
                let left = (-1, 0);

                if board.can_move(left, &cur_piece) {
                    cur_piece.move_piece(left);
                }
            }
            _ => panic!("Invalid move"),
        }

        let down = (0, -1);

        if board.can_move(down, &cur_piece) {
            cur_piece.move_piece(down);
        } else {
            board.land_piece(&cur_piece);
            num_landed += 1;
            remaining_pieces -= 1;

            if move_index == 0 {
                // Hit the beginning of the cycle
                if start_height == 0 {
                    start_height = board.get_tower_height();
                    num_landed_before_cycle = num_landed;
                }
                // Hit the end of the cycle
                else {
                    let cycle_length = num_landed - num_landed_before_cycle;
                    cycle_height = board.get_tower_height() - start_height;
                    let num_cycles = (n - num_landed_before_cycle) / cycle_length;
                    total_height += cycle_height * num_cycles + start_height;
                    remaining_pieces = (n - num_landed_before_cycle) % cycle_length;
                }
            }

            // Spawn next shape
            let next_shape = &shapes[num_landed % 5];
            let spawn_y = board.get_tower_height() + 2 + next_shape.len();
            cur_piece = Piece {
                x: 2,
                y: spawn_y as isize,
                shape: next_shape,
            };

            if spawn_y >= board.grid.len() {
                board.expand(spawn_y - board.grid.len() + 1);
            }
        }
        move_index = (move_index + 1) % moves.len();
    }
    // There was a cycle
    if total_height > 0 {
        return total_height + board.get_tower_height() - cycle_height - start_height;
    }
    // No cycle
    board.get_tower_height()
}

struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Board {
        let mut vec = Vec::new();
        for _ in 0..4 {
            vec.push(['.'].repeat(GRID_WIDTH));
        }
        Board { grid: vec }
    }

    fn print_board(&self, piece: &Piece) {
        let mut grid_copy = self.grid.clone();
        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[0].len() {
                grid_copy[piece.y as usize - row][piece.x as usize + col] = piece.shape[row][col];
            }
        }
        println!(
            "{}\n",
            grid_copy
                .iter()
                .rev()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn expand(&mut self, amount: usize) {
        for _ in 0..amount {
            self.grid.push(['.'].repeat(GRID_WIDTH));
        }
    }

    fn can_move(&self, direction: (isize, isize), piece: &Piece) -> bool {
        let new_x = piece.x + direction.0;
        let new_y = piece.y + direction.1;

        if new_x < 0 || new_x >= GRID_WIDTH as isize || new_y < 0 {
            return false;
        }

        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[0].len() {
                if piece.shape[row][col] == '#' {
                    if (new_y - row as isize) < 0 || new_x as usize + col >= GRID_WIDTH {
                        return false;
                    }
                    if self.grid[new_y as usize - row][new_x as usize + col] == '#' {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn land_piece(&mut self, piece: &Piece) {
        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[0].len() {
                if piece.shape[row][col] == '#' {
                    self.grid[piece.y as usize - row][piece.x as usize + col] = '#';
                }
            }
        }
    }

    fn get_tower_height(&self) -> usize {
        for (i, row) in self.grid.iter().rev().enumerate() {
            if row.contains(&'#') {
                return self.grid.len() - i;
            }
        }
        0
    }
}

#[derive(Debug)]
struct Piece<'a> {
    x: isize, // The coordinates of the top left of the piece
    y: isize,
    shape: &'a Vec<Vec<char>>,
}

impl Piece<'_> {
    fn move_piece(&mut self, direction: (isize, isize)) {
        self.x += direction.0;
        self.y += direction.1;
    }
}
