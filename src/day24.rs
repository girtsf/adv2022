use std::collections::HashSet;

use adv2022::{read_input, Pos};
use itertools::Itertools;

#[derive(Debug)]
struct Blizzard {
    pos: Pos,
    dir: Pos,
}

#[derive(Debug)]
struct Map {
    blizzards: Vec<Blizzard>,
    busy: HashSet<Pos>,
    width: isize,
    height: isize,

    starting_pos: Pos,
    ending_pos: Pos,

    poses: HashSet<Pos>,
    minute: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let lines = input.lines().collect_vec();
        let mut blizzards = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let dir = match c {
                    '>' => Pos(0, 1),
                    '<' => Pos(0, -1),
                    '^' => Pos(-1, 0),
                    'v' => Pos(1, 0),
                    '.' | '#' => continue,
                    _ => panic!("unexpected char"),
                };
                let pos = Pos(y as isize - 1, x as isize - 1);
                blizzards.push(Blizzard { pos, dir });
            }
        }
        let height = lines.len() as isize - 2;
        let width = lines[0].len() as isize - 2;
        Map {
            busy: Map::make_busy(&blizzards),
            blizzards,
            height,
            width,
            starting_pos: Pos(-1, 0),
            ending_pos: Pos(height, width - 1),
            poses: HashSet::from([Pos(-1, 0)]),
            minute: 0,
        }
    }

    fn make_busy(blizzards: &[Blizzard]) -> HashSet<Pos> {
        let mut out: HashSet<Pos> = HashSet::new();
        for b in blizzards {
            out.insert(b.pos);
        }
        out
    }

    fn is_valid_pos(&self, pos: &Pos) -> bool {
        pos == &self.starting_pos
            || pos == &self.ending_pos
            || (pos.0 >= 0 && pos.0 < self.height && pos.1 >= 0 && pos.1 < self.width)
    }

    fn blow(&mut self) {
        for b in self.blizzards.iter_mut() {
            b.pos = b.pos + b.dir;
            if b.pos.0 < 0 {
                b.pos.0 = self.height - 1;
            } else if b.pos.0 >= self.height {
                b.pos.0 = 0;
            } else if b.pos.1 < 0 {
                b.pos.1 = self.width - 1;
            } else if b.pos.1 >= self.width {
                b.pos.1 = 0;
            }
        }
        self.busy = Map::make_busy(&self.blizzards);
    }

    fn do_minute(&mut self) {
        self.blow();
        self.minute += 1;

        let mut new_poses = HashSet::<Pos>::new();
        for pos in self.poses.iter() {
            // dbg!(pos);
            for d in [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1), Pos(0, 0)] {
                let new_pos = *pos + d;
                if !self.is_valid_pos(&new_pos) || self.busy.contains(&new_pos) {
                    continue;
                }
                // dbg!(&new_pos);
                new_poses.insert(new_pos);
            }
        }
        self.poses = new_poses;
    }

    fn draw(&self) {
        println!("-- {} --", self.minute);
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos(y, x);
                if self.busy.contains(&pos) {
                    assert!(!self.poses.contains(&pos));
                    print!("~");
                } else if self.poses.contains(&pos) {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn find_path(&mut self, to: &Pos) -> usize {
        while !self.poses.contains(to) {
            // self.draw();
            self.do_minute();
            dbg!(&self.minute);
        }
        self.poses = HashSet::from([*to]);
        self.minute
    }
}

#[derive(Debug)]
struct States {}

fn main() {
    let mut map = Map::parse(&read_input());
    map.draw();
    map.find_path(&map.ending_pos.clone());
    let part1 = map.minute;
    dbg!(&part1);
    dbg!(&map);
    map.find_path(&map.starting_pos.clone());
    map.find_path(&map.ending_pos.clone());
    let part2 = map.minute;
    dbg!(&part2);
}
