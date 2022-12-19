use std::{collections::BTreeMap, ops::Range};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

mod lib;

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct SensorInfo {
    sensor: Pos,
    beacon: Pos,
}

impl SensorInfo {
    fn parse_line(line: &str) -> SensorInfo {
        lazy_static! {
            // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            static ref RE: Regex =
                Regex::new(r"^Sensor at x=(?P<sx>.+), y=(?P<sy>.+): closest beacon is at x=(?P<bx>.+), y=(?P<by>.+)$")
                    .unwrap();
        }
        let c = RE.captures(line).unwrap();
        let sensor = Pos {
            x: c.name("sx").unwrap().as_str().parse().unwrap(),
            y: c.name("sy").unwrap().as_str().parse().unwrap(),
        };
        let beacon = Pos {
            x: c.name("bx").unwrap().as_str().parse().unwrap(),
            y: c.name("by").unwrap().as_str().parse().unwrap(),
        };
        SensorInfo { sensor, beacon }
    }

    /// Returns a range where a beacon cannot be present.
    fn get_no_beacon_range(&self, y: isize) -> Range<isize> {
        let r = self.get_empty_range(y);
        // For part 1, exclude the beacon. It must be at one of the ends.
        if self.beacon.y == y {
            if r.start == self.beacon.x {
                r.start +1 .. r.end
            } else {
                assert_eq!(r.end - 1, self.beacon.x);
                r.start .. r.end - 1
            }
        } else {
            r
        }
    }

    /// Returns a range that is empty (new beacon cannot be present, or existing beacon is
    /// present).
    fn get_empty_range(&self, y: isize) -> Range<isize> {
        let md = self.sensor.manhattan_distance(&self.beacon);
        let dy = (self.sensor.y - y).abs();
        if dy > md {
            return 0..0; // empty range
        }
        let from = self.sensor.x - (md - dy);
        let to = self.sensor.x + (md - dy) + 1;
        from..to
    }
}

#[derive(Debug)]
struct State {
    infos: Vec<SensorInfo>,
}

/// Insert ranges, get count of elements spanned by the ranges.
#[derive(Debug, Default)]
struct RangeUnion(BTreeMap<isize, isize>);

impl RangeUnion {
    fn add(&mut self, r: Range<isize>) {
        if r.is_empty() {
            return;
        }
        *self.0.entry(r.start).or_insert(0) += 1;
        *self.0.entry(r.end).or_insert(0) -= 1;
    }

    fn count(&self) -> usize {
        let mut count = 0isize;
        let mut in_ranges = 0isize;
        let mut last: Option<isize> = None;
        for (i, d) in self.0.iter() {
            if let Some(last) = last {
                if in_ranges != 0 {
                    count += i - last;
                }
            }
            in_ranges += d;
            assert!(in_ranges >= 0);
            last = Some(*i);
        }
        count as usize
    }

    fn get_singular_overlap(r1: &Range<isize>, r2: &Range<isize>) -> Option<isize> {
        // dbg!(r1, r2);
        fn check(r1: &Range<isize>, r2: &Range<isize>) -> Option<isize> {
            if (r1.end - r2.start) == 1 {
                // Ends overlap by 1.
                Some(r1.end - 1)
            } else if (r1.end - r1.start) == 1 && r2.contains(&r1.start) {
                // One is size=1 and is contained in the other.
                Some(r1.start)
            } else {
                None
            }
        }

        check(r1, r2).or_else(|| check(r2, r1))
    }

    fn find_singular_missing_item_in_range(&self, search_range: &Range<isize>) -> Option<isize> {
        let mut in_ranges = 0isize;
        let mut last: Option<isize> = None;
        for (i, d) in self.0.iter() {
            if let Some(last) = last {
                // Look for overlap between search_range & [last, i).
                if in_ranges == 0 {
                    if let Some(overlap) = Self::get_singular_overlap(search_range, &(last..*i)) {
                        return Some(overlap);
                    }
                }
            }
            in_ranges += d;
            assert!(in_ranges >= 0);
            last = Some(*i);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_union() {
        let mut r = RangeUnion::default();
        assert_eq!(r.count(), 0);
        r.add(0..1);
        assert_eq!(r.count(), 1);
        r.add(0..2);
        assert_eq!(r.count(), 2);
        r.add(-2..3);
        assert_eq!(r.count(), 5);
        r.add(100..200);
        assert_eq!(r.count(), 105);
    }
}

impl State {
    fn parse(input: &str) -> State {
        State {
            infos: input
                .lines()
                .map(|line| SensorInfo::parse_line(line))
                .collect_vec(),
        }
    }

    fn get_no_beacon_count(&self, y: isize) -> usize {
        let mut ru = RangeUnion::default();
        self.infos.iter().for_each(|i| {
            ru.add(i.get_no_beacon_range(y));
        });
        ru.count()
    }

    fn make_empty_range_union_for_y(&self, y: isize) -> RangeUnion {
        let mut ru = RangeUnion::default();
        self.infos.iter().for_each(|i| {
            ru.add(i.get_empty_range(y));
        });
        ru
    }


    fn find_beacon(&self, search_range: &Range<isize>) -> Pos {
        for y in search_range.clone() {
            let ru = self.make_empty_range_union_for_y(y);
            if let Some(x) = ru.find_singular_missing_item_in_range(search_range) {
                dbg!(&ru);
                return Pos { x, y };
            }
        }
        panic!("failed to find beacon");
    }
}

fn main() {
    let input = lib::read_input();
    let state = State::parse(&input);
    let nbc = state.get_no_beacon_count(2000000);
    dbg!(&nbc);
    let beacon = state.find_beacon(&(0..4000001));
    dbg!(&beacon);
    dbg!(beacon.x * 4000000 + beacon.y);
}
