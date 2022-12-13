use std::collections::HashSet;

mod lib;

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Pos(isize, isize);

#[derive(Debug, Default)]
struct State {
    head: Pos,
    tail: Pos,
    tail_visited: HashSet<Pos>,
}

impl State {
    fn parse_and_do_move(&mut self, line: &str) {
        let (dir, count) = line.split_once(" ").unwrap();
        self.do_move(dir.chars().next().unwrap(), count.parse().unwrap());
    }

    fn do_move(&mut self, dir: char, count: isize) {
        for _ in 0..count {
            self.move_head(dir);
            self.move_tail();
        }
        dbg!(dir, count);
    }

    fn move_head(&mut self, dir: char) {
        let (dy, dx): (isize, isize) = match dir {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("invalid dir"),
        };
        self.head.0 += dy;
        self.head.1 += dx;
    }

    fn move_tail(&mut self) {
        let dy = self.head.0 - self.tail.0;
        let dx = self.head.1 - self.tail.1;
        if dy.abs() > 1 || dx.abs() > 1 {
            self.tail.0 += dy.signum();
            self.tail.1 += dx.signum();
        }
        self.tail_visited.insert(self.tail.clone());
    }
}

fn main() {
    let input = lib::read_input();
    let mut state = State::default();
    input.lines().for_each(|line| state.parse_and_do_move(line));
    dbg!(&state);
    dbg!(state.tail_visited.len());
}
