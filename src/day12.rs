use std::collections::{HashMap, HashSet, VecDeque};

mod lib;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos(isize, isize);

#[derive(Debug)]
struct Map {
    heights: HashMap<Pos, isize>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut start = None;
        let mut end = None;
        let mut heights = HashMap::new();

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                let pos = Pos(row as isize, col as isize);
                match c {
                    'S' => {
                        start = Some(pos.clone());
                        heights.insert(pos, 0);
                    }
                    'E' => {
                        end = Some(pos.clone());
                        heights.insert(pos, 25);
                    }
                    'a'..='z' => {
                        heights.insert(pos, c as isize - 'a' as isize);
                    }
                    _ => panic!("invalid char in input: {}", c),
                }
            })
        });

        Map {
            heights,
            start: start.unwrap(),
            end: end.unwrap(),
            // ..Default::default()
        }
    }

    fn find_shortest_path(&self) -> usize {
        let mut visited: HashSet<Pos> = HashSet::from_iter([self.start.clone()]);
        let mut queue: VecDeque<(usize, Pos)> = VecDeque::from_iter([(0, self.start.clone())]);

        while !queue.is_empty() {
            let (steps, pos) = queue.pop_front().unwrap();
            if pos == self.end {
                return steps;
            }
            // dbg!(&steps, &pos);
            for (d_row, d_col) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                let maybe_pos = Pos(pos.0 + d_row, pos.1 + d_col);
                if visited.contains(&maybe_pos) {
                    continue;
                }
                let this_height = self.heights.get(&pos).unwrap();
                if let Some(other_height) = self.heights.get(&maybe_pos) {
                    if other_height - this_height <= 1 {
                        visited.insert(maybe_pos.clone());
                        queue.push_back((steps + 1, maybe_pos));
                    }
                }
            }
        }
        // Did not find a path.
        usize::MAX
    }
}

fn main() {
    let input = lib::read_input();
    let mut map = Map::parse(&input);
    // dbg!(&map);
    // Part 1:
    let part1 = map.find_shortest_path();
    dbg!(&part1);
    // Part 2:
    // Find all possible starting locations with height 'a':
    let a_poses: Vec<Pos> = map
        .heights
        .iter()
        .filter_map(|(pos, height)| {
            if height == &0 {
                Some(pos.clone())
            } else {
                None
            }
        })
        .collect();
    // dbg!(&a_poses);
    let part2 = a_poses
        .into_iter()
        .map(|pos| {
            map.start = pos;
            map.find_shortest_path()
        })
        .min()
        .unwrap();
    dbg!(&part2);
}
