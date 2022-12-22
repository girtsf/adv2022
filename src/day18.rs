use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

mod lib;

type Pos3 = (isize, isize, isize);
type Voxels = HashSet<Pos3>;

const SIDES: [Pos3; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn part1(cubes: &Voxels) -> usize {
    cubes
        .iter()
        .map(|&(x, y, z)| {
            SIDES
                .iter()
                .filter(|(dx, dy, dz)| !cubes.contains(&(x + dx, y + dy, z + dz)))
                .count()
        })
        .sum::<usize>()
}

/// Flood-fills space that's 1 cube outside of bounding box of cubes.
fn find_outside(cubes: &Voxels) -> Voxels {
    let mut outside = Voxels::new();
    // Find bounding box + 1.
    let min_x = cubes.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let max_x = cubes.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let min_y = cubes.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let max_y = cubes.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let min_z = cubes.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let max_z = cubes.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    dbg!(&min_x, &max_x, &min_y, &max_y, &min_z, &max_z);

    // Start from a corner and flood-fill the outside space.
    let mut todo: VecDeque<Pos3> = VecDeque::from_iter([(min_x, min_y, min_z)]);
    while let Some((x, y, z)) = todo.pop_front() {
        for (dx, dy, dz) in SIDES {
            let new_pos = (x + dx, y + dy, z + dz);
            let (new_x, new_y, new_z) = new_pos;
            // Throw out positions that are outside our bounding_box+1.
            if new_x < min_x
                || new_x > max_x
                || new_y < min_y
                || new_y > max_y
                || new_z < min_z
                || new_z > max_z
            {
                continue;
            }
            // Throw out stuff that's part of the scanned voxels or already known to be outside.
            if cubes.contains(&new_pos) || outside.contains(&new_pos) {
                continue;
            }
            // Otherwise, new_pos is outside.
            outside.insert(new_pos);
            todo.push_back(new_pos);
        }
    }
    outside
}

fn part2(cubes: &Voxels) -> usize {
    let outside = find_outside(&cubes);
    cubes
        .iter()
        .map(|&(x, y, z)| {
            SIDES
                .iter()
                .filter(|(dx, dy, dz)| {
                    let new_pos = (x + dx, y + dy, z + dz);
                    !cubes.contains(&new_pos) && outside.contains(&new_pos)
                })
                .count()
        })
        .sum::<usize>()
}

fn main() {
    let input = lib::read_input();
    let cubes = input
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect_tuple::<Pos3>()
                .unwrap()
        })
        .collect::<Voxels>();
    dbg!(&part1(&cubes));
    dbg!(&part2(&cubes));
}
