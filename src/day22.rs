use std::collections::HashMap;

use adv2022::read_input;

// (y, x), top left corner is (0, 0)
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos(isize, isize);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const FACINGS: [Pos; 4] = [
    Pos(0, 1),  // 0: right
    Pos(1, 0),  // 1: down
    Pos(0, -1), // 2: left
    Pos(-1, 0), // 3: up
];

type BoundMap = HashMap<isize, isize>;

#[derive(Debug)]
struct Map {
    tiles: HashMap<Pos, char>,
    // Smallest x in given row, etc.
    min_x: BoundMap,
    max_x: BoundMap,
    min_y: BoundMap,
    max_y: BoundMap,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut tiles = HashMap::new();
        let mut min_x = BoundMap::new();
        let mut max_x = BoundMap::new();
        let mut min_y = BoundMap::new();
        let mut max_y = BoundMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                let pos = Pos(y as isize, x as isize);
                tiles.insert(pos, c);
                min_x
                    .entry(y as isize)
                    .and_modify(|x2| *x2 = (*x2).min(x as isize))
                    .or_insert(x as isize);
                max_x
                    .entry(y as isize)
                    .and_modify(|x2| *x2 = (*x2).max(x as isize))
                    .or_insert(x as isize);
                min_y
                    .entry(x as isize)
                    .and_modify(|y2| *y2 = (*y2).min(y as isize))
                    .or_insert(y as isize);
                max_y
                    .entry(x as isize)
                    .and_modify(|y2| *y2 = (*y2).max(y as isize))
                    .or_insert(y as isize);
            }
        }
        Map {
            tiles,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Move(isize),
    R,
    L,
}

#[derive(Debug)]
struct Path(Vec<Cmd>);

impl Path {
    fn parse(input: &str) -> Path {
        let mut out = Vec::<Cmd>::new();
        let mut acc = 0isize;
        for c in input.trim().chars() {
            if !c.is_digit(10) && acc != 0 {
                out.push(Cmd::Move(acc));
                acc = 0;
            }
            if c == 'L' {
                out.push(Cmd::L);
            } else if c == 'R' {
                out.push(Cmd::R);
            } else {
                assert!(c.is_digit(10));
                acc = acc * 10 + (c as isize - '0' as isize);
            }
        }
        if acc != 0 {
            out.push(Cmd::Move(acc));
        }
        Path(out)
    }
}

#[derive(Debug)]
struct State {
    map: Map,
    pos: Pos,
    facing: usize, // 0..3 in FACINGS array
}

impl State {
    fn new(map: Map) -> State {
        // "You begin the path in the leftmost open tile of the top row of tiles."
        let pos = map
            .tiles
            .iter()
            .filter(|(Pos(y, _), v)| *y == 0 && **v == '.')
            .min_by_key(|(Pos(_, x), _)| *x)
            .unwrap()
            .0
            .clone();

        State {
            map,
            pos,
            facing: 0,
        }
    }

    fn follow_path(&mut self, path: &Path) {
        for cmd in path.0.iter() {
            dbg!(self.pos, self.facing, &cmd);
            match cmd {
                Cmd::Move(count) => {
                    for i in 0..*count {
                        dbg!(self.pos, &i);
                        let mut next_pos = self.pos + FACINGS[self.facing];
                        if !self.map.tiles.contains_key(&next_pos) {
                            // Wrapping.
                            next_pos = match self.facing {
                                0 => Pos(next_pos.0, *self.map.min_x.get(&next_pos.0).unwrap()),
                                1 => Pos(*self.map.min_y.get(&next_pos.1).unwrap(), next_pos.1),
                                2 => Pos(next_pos.0, *self.map.max_x.get(&next_pos.0).unwrap()),
                                3 => Pos(*self.map.max_y.get(&next_pos.1).unwrap(), next_pos.1),
                                _ => panic!("invalid facing"),
                            }
                        }

                        match self.map.tiles.get(&next_pos).unwrap() {
                            '#' => {
                                break;
                            }
                            '.' => {
                                self.pos = next_pos;
                            }
                            _ => {
                                panic!("invalid map char");
                            }
                        }
                    }
                }
                Cmd::R => {
                    self.facing = (self.facing + 1) % 4;
                }
                Cmd::L => {
                    self.facing = (self.facing + 3) % 4;
                }
            }
        }
    }

    fn final_password(&self) -> isize {
        (self.pos.0 + 1) * 1000 + (self.pos.1 + 1) * 4 + self.facing as isize
    }
}

fn main() {
    let input = read_input();
    let (map_input, path_input) = input.split_once("\n\n").unwrap();
    let mut state = State::new(Map::parse(map_input));
    let path = Path::parse(path_input);
    dbg!(&state, &path);
    state.follow_path(&path);
    dbg!(&state);
    dbg!(&state.final_password());
}
