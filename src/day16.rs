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
    // Only for part 2.
    elephant_location: Option<String>,
    valves_open: BTreeSet<String>,
    pressure_per_minute: usize,
}

impl State {
    fn new(location: &str, elephant: bool) -> State {
        State {
            location: location.to_owned(),
            elephant_location: if elephant {
                Some(location.to_owned())
            } else {
                None
            },
            valves_open: BTreeSet::new(),
            pressure_per_minute: 0,
        }
    }
    /// Creates a new state that has current valve opened.
    fn open_valve(&self, loc: &str, rate: usize) -> State {
        let mut new_state = self.clone();
        assert!(new_state.valves_open.insert(loc.to_owned()));
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
    /// Creates a new state with a different location for the elephant.
    fn move_elephant_to(&self, loc: &str) -> State {
        assert!(self.elephant_location.is_some());
        State {
            elephant_location: Some(loc.to_owned()),
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

    fn new_start(elephant: bool) -> StateUnion {
        StateUnion {
            pressure_by_state: HashMap::from_iter([(State::new("AA", elephant), 0)]),
        }
    }

    fn maybe_update_state(&mut self, state: State, pressure: usize) {
        let v = self.pressure_by_state.entry(state).or_insert(0);
        *v = (*v).max(pressure);
    }

    fn next_minute(&self, cave: &Cave) -> StateUnion {
        let mut next = StateUnion::new();
        for (state, pressure) in self.pressure_by_state.iter() {
            // dbg!(&state, &pressure);
            let current_valve = cave.valves.get(&state.location).unwrap();
            let new_pressure = state.pressure_per_minute + pressure;

            let mut new_states: Vec<State> = Vec::new();
            // If we are in a location with non-zero-rate valve that hasn't been opened yet, try
            // opening it.
            if !state.valves_open.contains(&state.location) && current_valve.flow_rate != 0 {
                new_states.push(state.open_valve(&state.location, current_valve.flow_rate));
            }
            // Or try moving to any of the exits.
            for exit in current_valve.exits.iter() {
                new_states.push(state.move_to(exit));
            }

            // Part 2 (only if we have an elephant).
            if let Some(ref elephant_location) = state.elephant_location {
                let elephant_valve = cave.valves.get(elephant_location).unwrap();
                let mut new_states2: Vec<State> = Vec::new();
                for new_state in new_states.iter() {
                    // If elephant is in a location with non-zero-rate valve that hasn't been
                    // opened yet, try opening it.
                    if !new_state.valves_open.contains(elephant_location)
                        && elephant_valve.flow_rate != 0
                    {
                        new_states2.push(
                            new_state.open_valve(elephant_location, elephant_valve.flow_rate),
                        );
                    }
                    // Or try moving the elephant to any of the exits.
                    for exit in elephant_valve.exits.iter() {
                        new_states2.push(new_state.move_elephant_to(exit));
                    }
                }
                new_states = new_states2;
            }

            new_states.into_iter().for_each(|new_state| {
                next.maybe_update_state(new_state, new_pressure);
            });
        }
        next
    }
}

fn plan(cave: &Cave, minutes: usize, elephant: bool) -> usize {
    let state_union = StateUnion::new_start(elephant);
    let mut prev = state_union;
    for i in 0..minutes {
        dbg!(&i, prev.pressure_by_state.len());
        let next = prev.next_minute(&cave);
        // dbg!(&i, &next);
        prev = next;
    }
    *prev.pressure_by_state.values().max().unwrap()
}

fn main() {
    let input = lib::read_input();
    let cave = Cave::parse(&input);
    // Part 1:
    dbg!(plan(&cave, 30, false));
    // Part 2:
    dbg!(plan(&cave, 26, true));
}
