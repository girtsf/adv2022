use std::{collections::HashSet, iter::repeat};

mod lib;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos(isize, isize);

#[derive(Debug)]
struct State {
    // Part 1: [0] is Head, [1] is tail.
    // Part 2: [0] is Head, [1] is 1, etc.
    rope: Vec<Pos>,
    tail_visited: HashSet<Pos>,
}

impl State {
    fn new(segments: usize) -> State {
        State {
            rope: repeat(Pos(0, 0)).take(segments).collect(),
            tail_visited: HashSet::default(),
        }
    }

    fn parse_and_do_move(&mut self, line: &str) {
        let (dir, count) = line.split_once(" ").unwrap();
        self.do_move(dir.chars().next().unwrap(), count.parse().unwrap());
    }

    fn do_move(&mut self, dir: char, count: isize) {
        for _ in 0..count {
            self.move_head(dir);
            for i in 1..self.rope.len() {
                self.move_tail(i);
            }
            self.tail_visited.insert(self.rope.last().unwrap().clone());
        }
    }

    fn move_head(&mut self, dir: char) {
        let (dy, dx): (isize, isize) = match dir {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("invalid dir"),
        };
        self.rope[0].0 += dy;
        self.rope[0].1 += dx;
    }

    fn move_tail(&mut self, i: usize) {
        let dy = self.rope[i - 1].0 - self.rope[i].0;
        let dx = self.rope[i - 1].1 - self.rope[i].1;
        if dy.abs() > 1 || dx.abs() > 1 {
            self.rope[i].0 += dy.signum();
            self.rope[i].1 += dx.signum();
        }
    }

    fn run(input: &str, segments: usize) -> usize {
        let mut state = Self::new(segments);
        input.lines().for_each(|line| state.parse_and_do_move(line));
        state.tail_visited.len()
    }
}

fn main() {
    let input = lib::read_input();
    // Part 1:
    dbg!(State::run(&input, 2));
    // Part 2:
    dbg!(State::run(&input, 10));
}
