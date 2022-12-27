use std::collections::HashMap;

use adv2022::{read_input, Between};
use ansi_term::{Color, Style};

// (y, x), top left corner is (0, 0)
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos(isize, isize);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

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
    width: usize,
    height: usize,
}

type WrappingFun = fn(&Map, &Pos, usize) -> (Pos, usize);

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
        let height = *max_x.keys().max().unwrap() as usize + 1;
        let width = *max_y.keys().max().unwrap() as usize + 1;
        Map {
            tiles,
            min_x,
            max_x,
            min_y,
            max_y,
            height,
            width,
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
struct State<'a> {
    map: &'a Map,
    pos: Pos,
    facing: usize, // 0..3 in FACINGS array
}

impl State<'_> {
    fn new(map: &Map) -> State {
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
            facing: RIGHT,
        }
    }

    fn part1_wrapping(map: &Map, pos: &Pos, facing: usize) -> (Pos, usize) {
        let next_pos = *pos + FACINGS[facing];
        (
            match facing {
                0 => Pos(next_pos.0, *map.min_x.get(&next_pos.0).unwrap()),
                1 => Pos(*map.min_y.get(&next_pos.1).unwrap(), next_pos.1),
                2 => Pos(next_pos.0, *map.max_x.get(&next_pos.0).unwrap()),
                3 => Pos(*map.max_y.get(&next_pos.1).unwrap(), next_pos.1),
                _ => panic!("invalid facing"),
            },
            facing,
        )
    }

    // CHEEZE ALERT: mapping is specific to my input, which looks like this:
    //   AB
    //   C
    //  DE
    //  F
    fn part2_wrapping(_map: &Map, pos: &Pos, facing: usize) -> (Pos, usize) {
        let next_pos = *pos + FACINGS[facing];
        // let side_size = map.max_x.get(&0).unwrap() - map.min_x.get(&0).unwrap() + 1;
        // dbg!(&side_size);

        // exit top of A: (-1, 50) .. (-1, 100) -> left of F
        if next_pos.0 == -1 && next_pos.1.between(&50, &100) {
            assert_eq!(facing, UP);
            return (Pos(150 + (next_pos.1 - 50), 0), RIGHT);
        }
        // exit left of F: (150, -1) .. (200, -1) -> top of A
        if next_pos.1 == -1 && next_pos.0.between(&150, &200) {
            assert_eq!(facing, LEFT);
            return (Pos(0, 50 + (next_pos.0 - 150)), DOWN);
        }
        // exit left of D: (100, -1) .. (150, -1) -> left of A (rev)
        if next_pos.1 == -1 && next_pos.0.between(&100, &150) {
            assert_eq!(facing, LEFT);
            return (Pos(49 - (next_pos.0 - 100), 50), RIGHT);
        }
        // exit left of A: (0, 49) .. (50, 49) -> left of D (rev)
        if next_pos.1 == 49 && next_pos.0.between(&0, &50) {
            assert_eq!(facing, LEFT);
            return (Pos(149 - next_pos.0, 0), RIGHT);
        }
        // exit right of F: (150, 50) .. (200, 50) -> bottom of E
        if facing == RIGHT && next_pos.1 == 50 && next_pos.0.between(&150, &200) {
            assert_eq!(facing, RIGHT);
            return (Pos(149, 50 + (next_pos.0 - 150)), UP);
        }
        // exit bottom of E: (150, 50) .. (150, 100) -> right of F
        if facing == DOWN && next_pos.0 == 150 && next_pos.1.between(&50, &100) {
            assert_eq!(facing, DOWN);
            return (Pos(150 + (next_pos.1 - 50), 49), LEFT);
        }
        // exit bottom of F: (200, 0) .. (200, 50) -> top of B
        if next_pos.0 == 200 && next_pos.1.between(&0, &50) {
            assert_eq!(facing, DOWN);
            return (Pos(0, 100 + next_pos.1), DOWN);
        }
        // exit top of B: (-1, 100) .. (-1, 150) -> bottom of F
        if next_pos.0 == -1 && next_pos.1.between(&100, &150) {
            assert_eq!(facing, UP);
            return (Pos(199, next_pos.1 - 100), UP);
        }
        // exit right of B: (0, 150) .. (50, 150) -> right of E (rev)
        if next_pos.1 == 150 && next_pos.0.between(&0, &50) {
            assert_eq!(facing, RIGHT);
            return (Pos(149 - next_pos.0, 99), LEFT);
        }
        // exit right of E: (100, 100) .. (150, 100) -> right of B (rev)
        if next_pos.1 == 100 && next_pos.0.between(&100, &150) {
            assert_eq!(facing, RIGHT);
            return (Pos(49 - (next_pos.0 - 100), 149), LEFT);
        }
        // exit left of C: (50, 49) .. (100, 49) -> top of D
        if facing == LEFT && next_pos.1 == 49 && next_pos.0.between(&50, &100) {
            assert_eq!(facing, LEFT);
            return (Pos(100, next_pos.0 - 50), DOWN);
        }
        // exit top of D: (99, 0) .. (99, 50) -> left of C (rec)
        if facing == UP && next_pos.0 == 99 && next_pos.1.between(&0, &50) {
            assert_eq!(facing, UP);
            return (Pos(50 + next_pos.1, 50), RIGHT);
        }
        // exit right of C: (50, 100) .. (100, 100) -> bottom of B
        if facing == RIGHT && next_pos.1 == 100 && next_pos.0.between(&50, &100) {
            assert_eq!(facing, RIGHT);
            return (Pos(49, 100 + (next_pos.0 - 50)), UP);
        }
        // exit bottom of B: (50, 100) .. (50, 150) -> right of C
        if facing == DOWN && next_pos.0 == 50 && next_pos.1.between(&100, &150) {
            assert_eq!(facing, DOWN);
            return (Pos(50 + (next_pos.1 - 100), 99), LEFT);
        }

        panic!("not implemented for {:?}", next_pos);
    }

    fn follow_path(&mut self, path: &Path, wrapping_fn: WrappingFun) {
        for cmd in path.0.iter() {
            // dbg!(self.pos, self.facing, &cmd);
            match cmd {
                Cmd::Move(count) => {
                    for _ in 0..*count {
                        // dbg!(self.pos, &i);
                        // let mut draw = false;
                        let mut next_pos = self.pos + FACINGS[self.facing];
                        let mut next_facing = self.facing;
                        if !self.map.tiles.contains_key(&next_pos) {
                            // println!("======================================================");
                            // println!(
                            //     "from pos={:?} facing={:?} via={:?}",
                            //     &self.pos, &self.facing, &next_pos);
                            // draw = true;
                            // self.draw();
                            (next_pos, next_facing) =
                                wrapping_fn(&self.map, &self.pos, self.facing);
                            if !self.map.tiles.contains_key(&next_pos) {
                                panic!("wrapping fun wrong: {:?}", &next_pos);
                            }
                            // println!(
                            //     "to next_pos={:?} to facing={:?}",
                            //     &next_pos, &next_facing
                            // );
                        }

                        match self.map.tiles.get(&next_pos).unwrap() {
                            '#' => {
                                // if draw {
                                //     println!(
                                //         "=============[ wall ]================================="
                                //     );
                                // }
                                break;
                            }
                            '.' => {
                                self.pos = next_pos;
                                self.facing = next_facing;
                                // if draw {
                                //     println!(
                                //         "=============[ moved ]================================="
                                //     );
                                //     self.draw();
                                //     println!(
                                //     "=============[ moving done ]================================="
                                // );
                                // }
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

    fn draw(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let pos = Pos(y as isize, x as isize);
                if self.pos == pos {
                    let c = ['>', 'v', '<', '^'][self.facing];
                    print!(
                        "{}",
                        Color::Black.on(Color::Red).paint(String::from_iter([c]))
                    );
                } else {
                    let c = self.map.tiles.get(&pos).cloned().unwrap_or(' ');
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    let input = read_input();
    let (map_input, path_input) = input.split_once("\n\n").unwrap();
    let path = Path::parse(path_input);
    let map = Map::parse(map_input);
    {
        // Part 1:
        let mut state = State::new(&map);
        state.follow_path(&path, State::part1_wrapping);
        // dbg!(&state);
        let part1 = state.final_password();
        dbg!(&part1);
    }
    {
        // Part 2:
        let mut state = State::new(&map);
        state.follow_path(&path, State::part2_wrapping);
        // dbg!(&state);
        let part2 = state.final_password();
        dbg!(&part2);
    }
}
