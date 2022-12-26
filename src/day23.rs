use std::collections::{HashMap, HashSet};

use adv2022::read_input;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos(isize, isize);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Map {
    elves: HashSet<Pos>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut elves = HashSet::<Pos>::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    elves.insert(Pos(y as isize, x as isize));
                }
            });
        });
        Map { elves }
    }

    fn can_go_north(elves: &HashSet<Pos>, pos: &Pos) -> Option<Pos> {
        let pos2 = *pos + Pos(-1, 0);
        if (-1..=1).all(|dx| !elves.contains(&(pos2 + Pos(0, dx)))) {
            Some(pos2)
        } else {
            None
        }
    }

    fn can_go_south(elves: &HashSet<Pos>, pos: &Pos) -> Option<Pos> {
        let pos2 = *pos + Pos(1, 0);
        if (-1..=1).all(|dx| !elves.contains(&(pos2 + Pos(0, dx)))) {
            Some(pos2)
        } else {
            None
        }
    }

    fn can_go_west(elves: &HashSet<Pos>, pos: &Pos) -> Option<Pos> {
        let pos2 = *pos + Pos(0, -1);
        if (-1..=1).all(|dy| !elves.contains(&(pos2 + Pos(dy, 0)))) {
            Some(pos2)
        } else {
            None
        }
    }

    fn can_go_east(elves: &HashSet<Pos>, pos: &Pos) -> Option<Pos> {
        let pos2 = *pos + Pos(0, 1);
        if (-1..=1).all(|dy| !elves.contains(&(pos2 + Pos(dy, 0)))) {
            Some(pos2)
        } else {
            None
        }
    }

    const CAN_GOES: [fn(&HashSet<Pos>, &Pos) -> Option<Pos>; 4] = [
        Map::can_go_north,
        Map::can_go_south,
        Map::can_go_west,
        Map::can_go_east,
    ];

    fn has_neighbors(&self, pos: &Pos) -> bool {
        for y in -1..=1 {
            for x in -1..=1 {
                if (x != 0 || y != 0) && self.elves.contains(&(*pos + Pos(y, x))) {
                    return true;
                }
            }
        }
        false
    }

    fn do_round(&mut self, round: usize) -> bool {
        let mut proposals: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut new_elves = HashSet::<Pos>::new();
        for e in self.elves.iter() {
            let mut found = false;
            if self.has_neighbors(e) {
                for i in 0..4 {
                    if let Some(new_pos) = Map::CAN_GOES[(round + i) % 4](&self.elves, e) {
                        proposals.entry(new_pos).or_default().push(*e);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                new_elves.insert(*e);
            }
        }
        // dbg!(&proposals);
        for (to, froms) in proposals.iter() {
            if froms.len() == 1 {
                new_elves.insert(*to);
            } else {
                new_elves.extend(froms.iter());
            }
        }
        if self.elves == new_elves {
            return true;
        } else {
            self.elves = new_elves;
            return false;
        }
    }

    fn empty_ground_tiles(&self) -> usize {
        // Find bounding box.
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for e in self.elves.iter() {
            min_x = min_x.min(e.1);
            max_x = max_x.max(e.1);
            min_y = min_y.min(e.0);
            max_y = max_y.max(e.0);
        }
        let area = (max_x - min_x + 1) * (max_y - min_y + 1);
        area as usize - self.elves.len()
    }
}

fn main() {
    let mut map = Map::parse(&read_input());
    // Part 1:
    for round in 0..10 {
        map.do_round(round);
    }
    dbg!(&map.empty_ground_tiles());
    // Part 2:
    let mut round = 10;
    loop {
        if map.do_round(round) {
            break;
        }
        round += 1;
    }
    dbg!(round + 1);
}
