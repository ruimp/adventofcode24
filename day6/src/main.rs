use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
// use std::{thread, time};

#[derive(Debug)]
struct Board {
    // pos = (row, column)
    pos: (usize, usize),
    // 0 = left  |  1 = down  |  2 = up  |  3 = right
    dimensions: (usize, usize),
    orientation: usize,
    obstructions: Vec<(usize, usize)>,
    path: Vec<(usize, usize)>,
    time: usize,
    exit: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = format!("t = {}\n", self.time);
        let mut board: String = String::from(title);
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if self.pos == (i, j) {
                    match self.orientation {
                        0 => board.push('<'),
                        1 => board.push('v'),
                        2 => board.push('^'),
                        3 => board.push('>'),
                        _ => panic!("Invalid orientation"),
                    }
                } else if self.obstructions.contains(&(i, j)) {
                    board.push('#');
                } else if self.path.contains(&(i, j)) {
                    board.push('X');
                } else {
                    board.push('.');
                }
            }
            board.push('\n');
        }
        write!(f, "{}", board)
    }
}

impl Board {
    fn evolve(&mut self) {
        match self.orientation {
            0 => self.move_left(),
            1 => self.move_down(),
            2 => self.move_up(),
            3 => self.move_right(),
            _ => panic!("Invalid orientation"),
        };
    }

    fn move_left(&mut self) {
        let row = self.pos.0;
        let col = self.pos.1.checked_sub(1).unwrap();
        let next_move: (usize, usize) = (row, col);
        if self.obstructions.contains(&next_move) {
            // orientation => up
            self.orientation = 2;
        } else {
            self.pos = next_move;
            self.time += 1;
            if !self.path.contains(&next_move) {
                self.path.push(next_move)
            }
        }
    }

    fn move_down(&mut self) {
        let row = self.pos.0.checked_add(1).unwrap();
        let col = self.pos.1;
        let next_move: (usize, usize) = (row, col);
        if self.obstructions.contains(&next_move) {
            // orientation down => left
            self.orientation = 0;
        } else {
            self.pos = next_move;
            self.time += 1;
            if !self.path.contains(&next_move) {
                self.path.push(next_move)
            }
        }
    }

    fn move_up(&mut self) {
        let row = self.pos.0.checked_sub(1).unwrap();
        let col = self.pos.1;
        let next_move: (usize, usize) = (row, col);
        if self.obstructions.contains(&next_move) {
            // orientation up => right
            self.orientation = 3;
        } else {
            self.pos = next_move;
            self.time += 1;
            if !self.path.contains(&next_move) {
                self.path.push(next_move)
            }
        }
    }

    fn move_right(&mut self) {
        let row = self.pos.0;
        let col = self.pos.1.checked_add(1).unwrap();
        let next_move: (usize, usize) = (row, col);
        if self.obstructions.contains(&next_move) {
            // orientation right => down
            self.orientation = 1;
        } else {
            self.pos = next_move;
            self.time += 1;
            if !self.path.contains(&next_move) {
                self.path.push(next_move)
            }
        }
    }

    fn check_exit(&mut self) {
        if (self.pos.1 == 0 && self.orientation == 0)
            || (self.pos.0 == self.dimensions.0.checked_sub(1).unwrap() && self.orientation == 1)
            || (self.pos.0 == 0 && self.orientation == 2)
            || (self.pos.1 == self.dimensions.1.checked_sub(1).unwrap() && self.orientation == 3)
        {
            self.exit = true;
        } else {
            self.exit = false;
        }
    }
}

fn get_board(filename: String) -> Result<Board> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let lines = buffer.lines();
    let mut obstructions: Vec<(usize, usize)> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut orientation: usize = 0;
    let mut n_rows: usize = 0;
    let mut n_cols: usize = 0;
    for (i, line) in lines.enumerate() {
        n_rows += 1;
        n_cols = 0;
        for (j, c) in line?.chars().enumerate() {
            n_cols += 1;
            match c {
                '#' => obstructions.push((i, j)),
                '<' => {
                    pos = (i, j);
                    orientation = 0;
                }
                'v' => {
                    pos = (i, j);
                    orientation = 1;
                }
                '^' => {
                    pos = (i, j);
                    orientation = 2;
                }
                '>' => {
                    pos = (i, j);
                    orientation = 3;
                }
                _ => continue,
            }
        }
    }
    let board = Board {
        pos: pos,
        dimensions: (n_rows, n_cols),
        orientation: orientation,
        obstructions: obstructions,
        path: vec![pos],
        time: 0,
        exit: false,
    };
    Ok(board)
}

fn main() {
    // let filename = String::from("test_input.txt");
    let filename = String::from("input.txt");
    let mut board = get_board(filename).unwrap();
    // println!("{}", &board);
    while !board.exit {
        board.evolve();
        board.check_exit();
        // println!("{}", &board);
        // thread::sleep(time::Duration::from_millis(150));
    }
    println!("Total steps: {}", board.time);
    let count_pos = board.path.iter().count();
    println!("Positions visited: {}", count_pos);
}
