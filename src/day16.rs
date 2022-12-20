mod lib;

use std::collections::{BTreeSet, HashMap};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    exits: Vec<String>,
}

impl Valve {
    fn parse(line: &str) -> Valve {
        lazy_static! {
            // Valve HH has flow rate=22; tunnel leads to valve GG
            // Valve II has flow rate=0; tunnels lead to valves AA, JJ
            static ref RE: Regex =
                Regex::new(r"^Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<exits>.*)$").unwrap();
        }
        let c = RE.captures(line).unwrap();
        Valve {
            name: c.name("name").unwrap().as_str().to_owned(),
            flow_rate: c.name("flow_rate").unwrap().as_str().parse().unwrap(),
            exits: c
                .name("exits")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_owned())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>,
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let valves = input
            .lines()
            .map(|line| {
                let valve = Valve::parse(line);
                (valve.name.clone(), valve)
            })
            .collect();
        Cave { valves }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    location: String,
    valves_open: BTreeSet<String>,
    pressure_per_minute: usize,
}

impl State {
    fn new(location: &str) -> State {
        State {
            location: location.to_owned(),
            valves_open: BTreeSet::new(),
            pressure_per_minute: 0,
        }
    }
    /// Creates a new state that has current valve opened.
    fn open_valve(&self, rate: usize) -> State {
        let mut new_state = self.clone();
        assert!(new_state.valves_open.insert(self.location.clone()));
        new_state.pressure_per_minute += rate;
        new_state
    }
    /// Creates a new state with a different location.
    fn move_to(&self, loc: &str) -> State {
        State {
            location: loc.to_owned(),
            ..self.clone()
        }
    }
}

#[derive(Debug)]
struct StateUnion {
    pressure_by_state: HashMap<State, usize>,
}

impl StateUnion {
    fn new() -> StateUnion {
        StateUnion {
            pressure_by_state: HashMap::new(),
        }
    }

    fn new_start() -> StateUnion {
        StateUnion {
            pressure_by_state: HashMap::from_iter([(State::new("AA"), 0)]),
        }
    }

    fn next_minute(&self, cave: &Cave) -> StateUnion {
        let mut next = StateUnion::new();
        for (state, pressure) in self.pressure_by_state.iter() {
            // dbg!(&state, &pressure);
            let current_valve = cave.valves.get(&state.location).unwrap();

            let new_pressure = state.pressure_per_minute;
            // If we are in a location with non-zero-rate valve that hasn't been opened yet, try
            // opening it.
            if !state.valves_open.contains(&state.location) && current_valve.flow_rate != 0 {
                let new_state = state.open_valve(current_valve.flow_rate);
                // XXX: refactor this update.
                next.pressure_by_state
                    .entry(new_state)
                    .and_modify(|v| {
                        *v = (*v).max(pressure + new_pressure);
                    })
                    .or_insert(pressure + new_pressure);
            }
            // Or try moving to any of the exits.
            for exit in current_valve.exits.iter() {
                // dbg!(exit);
                let new_state = state.move_to(exit);
                // XXX: refactor this update.
                next.pressure_by_state
                    .entry(new_state)
                    .and_modify(|v| {
                        *v = (*v).max(pressure + new_pressure);
                    })
                    .or_insert(pressure + new_pressure);
            }
        }
        next
    }
}

fn main() {
    let input = lib::read_input();
    let cave = Cave::parse(&input);
    // dbg!(&cave);
    let state_union = StateUnion::new_start();
    // dbg!(&state_union);
    let mut prev = state_union;
    for i in 0..30 {
        dbg!(&i, prev.pressure_by_state.len());
        let next = prev.next_minute(&cave);
        // dbg!(&i, &next);
        prev = next;
    }
    dbg!(&prev.pressure_by_state.values().max());
}
