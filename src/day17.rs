use std::iter::{repeat, repeat_with};

use itertools::Itertools;

mod lib;

const CHAMBER_WIDTH: usize = 7;

#[derive(Debug)]
struct Piece {
    shape: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Piece {
    fn parse(lines: &str) -> Piece {
        let shape = lines
            .lines()
            .rev()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("invalid char"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Piece {
            width: shape[0].len(),
            height: shape.len(),
            shape,
        }
    }
}

#[derive(Debug)]
struct Pos {
    // Bottom left corner of a piece.
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct State {
    pieces: Vec<Piece>,
    moves: Vec<char>,
    move_idx: usize,
    chamber: Vec<Vec<bool>>,
    piece_pos: Pos,
    piece_idx: usize,
    piece_count: usize,
    tower_height: usize,
}

impl State {
    fn new(pieces: Vec<Piece>, moves: &str) -> State {
        let piece_pos = Pos { x: 2, y: 3 };
        let piece_idx = 0;
        let chamber_height = piece_pos.y + pieces[piece_idx].height;

        State {
            pieces,
            moves: moves.trim().chars().collect(),
            move_idx: 0,
            chamber: repeat_with(|| repeat(false).take(CHAMBER_WIDTH).collect())
                .take(chamber_height)
                .collect(),
            piece_pos,
            piece_idx,
            piece_count: 1,
            tower_height: 0,
        }
    }

    fn cur_piece(&self) -> &Piece {
        &self.pieces[self.piece_idx]
    }

    fn get_next_move_dx(&mut self) -> isize {
        let dx = match self.moves[self.move_idx] {
            '>' => 1,
            '<' => -1,
            x => panic!("invalid move: {}", x),
        };
        self.move_idx = (self.move_idx + 1) % self.moves.len();
        dx
    }

    fn can_fit(&self, pos: &Pos) -> bool {
        let piece = self.cur_piece();
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.shape[y][x] && self.chamber[y + pos.y][x + pos.x] {
                    return false;
                }
            }
        }
        true
    }

    fn do_move(&mut self) {
        self.do_jet_move();
        self.do_fall();
    }

    fn do_jet_move(&mut self) {
        // Figure out which direction the wind blows.
        let move_dx = self.get_next_move_dx();
        println!("move_dx: {}", &move_dx);
        // Can we move the piece in that direction?
        // Would the piece be out of bounds?
        if move_dx < 0 && self.piece_pos.x == 0 {
            println!("can't move left - wall");
            return;
        }
        let piece = self.cur_piece();
        if move_dx > 0 && (self.piece_pos.x + piece.width) >= CHAMBER_WIDTH {
            println!("can't move right - wall");
            return;
        }
        // Would the piece crash into anything?
        let new_piece_pos = Pos {
            x: (self.piece_pos.x as isize + move_dx) as usize,
            ..self.piece_pos
        };
        if self.can_fit(&new_piece_pos) {
            println!("moving {}", move_dx);
            self.piece_pos = new_piece_pos;
        } else {
            println!("can't move {} because of overlap", move_dx);
        }
    }

    fn do_fall(&mut self) {
        // Can it fall?
        if self.piece_pos.y == 0 {
            self.come_to_rest();
            return;
        }

        let new_piece_pos = Pos {
            y: self.piece_pos.y - 1,
            ..self.piece_pos
        };
        if !self.can_fit(&new_piece_pos) {
            self.come_to_rest();
            return;
        }
        println!("moving down");
        self.piece_pos = new_piece_pos;
    }

    fn come_to_rest(&mut self) {
        println!("come_to_rest");
        // "Cement" the piece in the piece.
        let piece = &self.pieces[self.piece_idx];
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.shape[y][x] {
                    assert!(!self.chamber[y + self.piece_pos.y][x + self.piece_pos.x]);
                    self.chamber[y + self.piece_pos.y][x + self.piece_pos.x] = true;
                }
            }
        }
        self.tower_height = self.tower_height.max(piece.height + self.piece_pos.y);
        // Spawn the next piece.
        self.piece_idx = (self.piece_idx + 1) % self.pieces.len();
        self.piece_count += 1;
        self.piece_pos.x = 2;
        self.piece_pos.y = self.tower_height + 3;

        // Extend the chamber, as needed.
        let new_piece = &self.pieces[self.piece_idx];
        let new_top_y = new_piece.height + self.piece_pos.y;
        while new_top_y > self.chamber.len() {
            self.chamber
                .push(repeat(false).take(CHAMBER_WIDTH).collect());
        }
    }

    fn draw(&self) {
        println!();

        let piece = self.cur_piece();

        for (y, row) in self.chamber.iter().enumerate().rev() {
            let maybe_in_piece_y = {
                if y >= self.piece_pos.y && y < (self.piece_pos.y + piece.height) {
                    Some(y - self.piece_pos.y)
                } else {
                    None
                }
            };

            print!("{:5} |", y);
            for (x, c) in row.iter().enumerate() {
                if let Some(in_piece_y) = maybe_in_piece_y {
                    if x >= self.piece_pos.x && x < (self.piece_pos.x + piece.width) {
                        let in_piece_x = x - self.piece_pos.x;
                        if piece.shape[in_piece_y][in_piece_x] {
                            print!("@");
                            continue;
                        }
                    }
                }
                print!("{}", if *c { '#' } else { '.' });
            }
            println!("|");
        }
        println!("      +-------+");
    }
}

fn main() {
    let piece_lines = std::fs::read_to_string("input/day17-pieces.txt").expect("read failed");
    let pieces = piece_lines
        .split("\n\n")
        .map(|lines| Piece::parse(lines))
        .collect_vec();
    // dbg!(&pieces);

    let input = lib::read_input();
    let mut state = State::new(pieces, &input);
    state.draw();
    while state.piece_count < 2023 {
        state.do_move();
        // state.draw();
    }
    state.draw();
    dbg!(state.tower_height);
}
