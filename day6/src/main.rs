use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::{thread, time};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Orientation {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum GameState {
    Continues,
    Exits,
    Loops,
}

#[derive(Debug, Clone)]
struct Board {
    dimensions: (usize, usize),
    pos: (usize, usize),
    orientation: Orientation,
    obstructions: HashSet<(usize, usize)>,
    path: HashSet<(usize, usize)>,
    history: HashSet<((usize, usize), Orientation)>,
    time: usize,
    state: GameState,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = format!("t = {}\n", self.time);
        let mut board: String = String::from(title);
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if self.pos == (i, j) {
                    match self.orientation {
                        Orientation::Left => board.push('<'),
                        Orientation::Up => board.push('^'),
                        Orientation::Right => board.push('>'),
                        Orientation::Down => board.push('v'),
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
    fn get_move(&mut self) {
        let mut move_row = self.pos.0;
        let mut move_col = self.pos.1;
        match self.orientation {
            Orientation::Left => move_col = move_col.wrapping_sub(1),
            Orientation::Up => move_row = move_row.wrapping_sub(1),
            Orientation::Right => move_col = move_col.wrapping_add(1),
            Orientation::Down => move_row = move_row.wrapping_add(1),
        }
        let next_move: (usize, usize) = (move_row, move_col);
        self.path.insert(self.pos);
        self.history.insert((self.pos, self.orientation));
        if self.obstructions.contains(&next_move) {
            self.rotate_clockwise();
        } else {
            self.pos = next_move;
        }
        self.time += 1;
    }

    fn rotate_clockwise(&mut self) {
        match self.orientation {
            Orientation::Left => self.orientation = Orientation::Up,
            Orientation::Up => self.orientation = Orientation::Right,
            Orientation::Right => self.orientation = Orientation::Down,
            Orientation::Down => self.orientation = Orientation::Left,
        }
    }

    fn check_exits(&self) -> bool {
        match self.orientation {
            Orientation::Left => self.pos.1 == 0,
            Orientation::Up => self.pos.0 == 0,
            Orientation::Right => self.pos.1 == self.dimensions.1.wrapping_sub(1),
            Orientation::Down => self.pos.0 == self.dimensions.0.wrapping_sub(1),
        }
    }

    fn check_loops(&self) -> bool {
        let state = (self.pos, self.orientation);
        self.history.contains(&state)
    }

    fn check_state(&mut self) {
        if self.check_exits() {
            self.state = GameState::Exits;
        } else if self.check_loops() {
            self.state = GameState::Loops;
        }
    }
}

fn get_board(filename: String) -> Option<Board> {
    let file = File::open(filename).ok()?;
    let buffer = BufReader::new(file);
    let lines = buffer.lines();
    let mut pos: (usize, usize) = (0, 0);
    let mut orientation: Orientation = Orientation::Left;
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();
    let mut n_rows: usize = 0;
    let mut n_cols: usize = 0;
    for (i, line) in lines.enumerate() {
        n_rows += 1;
        n_cols = 0;
        for (j, c) in line.ok()?.chars().enumerate() {
            n_cols += 1;
            match c {
                '#' => {
                    obstructions.insert((i, j));
                }
                '<' => {
                    pos = (i, j);
                    orientation = Orientation::Left;
                }
                '^' => {
                    pos = (i, j);
                    orientation = Orientation::Up;
                }
                '>' => {
                    pos = (i, j);
                    orientation = Orientation::Right;
                }
                'v' => {
                    pos = (i, j);
                    orientation = Orientation::Down;
                }
                _ => continue,
            }
        }
    }
    let path: HashSet<(usize, usize)> = HashSet::from([pos]);
    let history: HashSet<((usize, usize), Orientation)> = HashSet::from([(pos, orientation)]);
    if (n_rows, n_cols) == (0, 0) {
        return None;
    }
    let board = Board {
        dimensions: (n_rows, n_cols),
        pos: pos,
        orientation: orientation,
        obstructions: obstructions,
        path: path,
        history: history,
        time: 0,
        state: GameState::Continues,
    };
    Some(board)
}

fn test_board(board: &Board, obs_pos: (usize, usize)) -> GameState {
    let mut obs_board = board.clone();
    obs_board.obstructions.insert(obs_pos);
    while obs_board.state == GameState::Continues {
        obs_board.get_move();
        obs_board.check_state();
    }
    obs_board.get_move();
    obs_board.state
}

fn main() {
    // let filename = String::from("test_input.txt");
    // let filename = String::from("test_input2.txt");
    let filename = String::from("input.txt");
    // println!("{}", &board);
    let board = match get_board(filename) {
        Some(board) => board,
        None => {
            println!("invalid board");
            return ();
        }
    };
    let mut end_board = board.clone();
    while end_board.state == GameState::Continues {
        end_board.get_move();
        end_board.check_state();
    }
    end_board.get_move();

    println!("base endgame: {:?}", end_board.state);
    println!("tiles visited: {:?}", end_board.path.len());
    let mut count: usize = 0;
    for i in 0..board.dimensions.0 {
        for j in 0..board.dimensions.1 {
            if end_board.path.contains(&(i, j)) && board.pos != (i, j) {
                if test_board(&board, (i, j)) == GameState::Loops {
                    count += 1;
                }
            }
        }
    }
    println!("1-alterations that loop: {:?}", count);
}
