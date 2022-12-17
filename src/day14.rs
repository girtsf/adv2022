use std::collections::HashMap;

mod lib;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn parse(s: &str) -> Pos {
        let (sx, sy) = s.split_once(',').unwrap();
        let x = sx.parse::<isize>().unwrap();
        let y = sy.parse::<isize>().unwrap();
        Pos { x, y }
    }
}

#[derive(Debug)]
enum What {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Cave {
    loc: HashMap<Pos, What>,
    largest_y: isize,
    abyss_y: isize,
    floor_y: isize,
}

impl Cave {
    /// Parses input line, returns rock positions.
    fn parse_input_line(line: &str) -> Vec<Pos> {
        let mut pos_iter = line.split(" -> ").map(|s| Pos::parse(s));
        let mut from = pos_iter.next().unwrap();
        let mut out = Vec::new();
        while let Some(to) = pos_iter.next() {
            // dbg!(&from, &to);
            let dx = (to.x - from.x).signum();
            let dy = (to.y - from.y).signum();
            // dbg!(&dy, &dx);

            let mut tmp = from.clone();
            while tmp != to {
                out.push(tmp.clone());
                tmp.x += dx;
                tmp.y += dy;
            }
            out.push(to.clone());
            from = to;
        }
        // dbg!(&out);
        out
    }

    /// Parses the whole input.
    fn parse(input: &str) -> Cave {
        let loc: HashMap<Pos, What> = input
            .lines()
            .flat_map(|line| Self::parse_input_line(line))
            .map(|pos| (pos, What::Rock))
            .collect();
        let largest_y = loc.iter().map(|(k, _)| k.y).max().unwrap();
        Cave {
            loc,
            largest_y,
            abyss_y: isize::MAX,
            floor_y: isize::MAX,
        }
    }

    /// Simulates a piece of sand falling from x=500, y=0, returns true iff it stayed in bounds.
    fn drop_sand(&mut self) -> bool {
        let mut pos = Pos { x: 500, y: 0 };
        if self.loc.contains_key(&pos) {
            // Can't even spawn, perhaps this can't happen.
            return false;
        }
        'outer: loop {
            if pos.y > self.abyss_y {
                // dbg!(pos, "into abyss!");
                // Falls into abyss.
                return false;
            }
            if pos.y >= (self.floor_y - 1) {
                // Part 2: resting on the floor.
                self.loc.insert(pos, What::Sand);
                return true;
            }
            for dx in [0, -1, 1] {
                let maybe = Pos {
                    y: pos.y + 1,
                    x: pos.x + dx,
                };
                if !self.loc.contains_key(&maybe) {
                    pos = maybe;
                    continue 'outer;
                }
            }
            // Can't fall any further.
            // dbg!("rest", &pos);
            self.loc.insert(pos, What::Sand);
            return true;
        }
    }
}

fn main() {
    let input = lib::read_input();
    {
        // Part 1.
        let mut cave = Cave::parse(&input);
        cave.abyss_y = cave.largest_y;
        let mut dropped = 0u32;
        while cave.drop_sand() {
            dropped += 1;
        }
        dbg!(&dropped);
    }
    {
        // Part 2.
        let mut cave = Cave::parse(&input);
        cave.floor_y = cave.largest_y + 2;
        let mut dropped = 0u32;
        while cave.drop_sand() {
            dropped += 1;
        }
        dbg!(&dropped);
    }
}
