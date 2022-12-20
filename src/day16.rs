mod lib;

use std::{collections::HashMap, mem::swap};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct ParsedValve {
    name: String,
    flow_rate: usize,
    exits: Vec<String>,
}

impl ParsedValve {
    fn parse(line: &str) -> ParsedValve {
        lazy_static! {
            // Valve HH has flow rate=22; tunnel leads to valve GG
            // Valve II has flow rate=0; tunnels lead to valves AA, JJ
            static ref RE: Regex =
                Regex::new(r"^Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<exits>.*)$").unwrap();
        }
        let c = RE.captures(line).unwrap();
        ParsedValve {
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
struct Valve {
    flow_rate: usize,
    exits: Vec<u8>,
}

#[derive(Debug)]
struct Cave {
    valves: Vec<Valve>,
    starting_location: u8,
    max_pressure_per_minute: usize,
    pressure_per_minute_per_valve_state: HashMap<u64, usize>,
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let parsed_valves = input
            .lines()
            .map(|line| ParsedValve::parse(line))
            .collect_vec();
        // Build a mapping from strings to ints.
        let mapping: HashMap<String, usize> = parsed_valves
            .iter()
            .enumerate()
            .map(|(i, pvalve)| (pvalve.name.clone(), i))
            .collect();
        let valves = parsed_valves
            .iter()
            .map(|pvalve| Valve {
                flow_rate: pvalve.flow_rate,
                exits: pvalve
                    .exits
                    .iter()
                    .map(|ex| *mapping.get(ex).unwrap() as u8)
                    .collect(),
            })
            .collect();
        Cave {
            valves,
            starting_location: *mapping.get("AA").unwrap() as u8,
            max_pressure_per_minute: parsed_valves.iter().map(|pvalve| pvalve.flow_rate).sum(),
            pressure_per_minute_per_valve_state: HashMap::new(),
        }
    }

    fn get_pressure_per_minute(&mut self, valves_open: u64) -> usize {
        *self
            .pressure_per_minute_per_valve_state
            .entry(valves_open)
            .or_insert_with(|| {
                self.valves
                    .iter()
                    .enumerate()
                    .map(|(i, valve)| {
                        if (1 << i) & valves_open != 0 {
                            valve.flow_rate
                        } else {
                            0
                        }
                    })
                    .sum()
            })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    location: u8,
    // Only used for part 2.
    elephant_location: u8,
    // Bitfield of which valves are open.
    valves_open: u64,
}

impl State {
    fn new(location: u8) -> State {
        State {
            location,
            elephant_location: location,
            valves_open: 0,
            // pressure_per_minute: 0,
        }
    }
    /// Creates a new state that has current valve opened.
    fn open_valve(&self, loc: u8) -> State {
        let bit = 1u64 << (loc as usize);
        assert_eq!(self.valves_open & bit, 0);
        State {
            valves_open: self.valves_open | bit,
            // pressure_per_minute: self.pressure_per_minute + rate,
            ..self.clone()
        }
    }
    /// Creates a new state with a different location.
    fn move_to(&self, location: u8) -> State {
        State {
            location,
            ..self.clone()
        }
    }
    /// Creates a new state with a different location for the elephant.
    fn move_elephant_to(&self, elephant_location: u8) -> State {
        State {
            elephant_location,
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

    fn new_start(starting_location: u8) -> StateUnion {
        StateUnion {
            pressure_by_state: HashMap::from_iter([(State::new(starting_location), 0)]),
        }
    }

    fn maybe_update_state(&mut self, mut state: State, pressure: usize, use_elephant: bool) {
        if use_elephant && state.elephant_location > state.location {
            swap(&mut state.elephant_location, &mut state.location);
        }

        let v = self.pressure_by_state.entry(state).or_insert(0);
        *v = (*v).max(pressure);
    }

    fn next_minute(&self, cave: &mut Cave, use_elephant: bool) -> StateUnion {
        let mut next = StateUnion::new();
        for (state, pressure) in self.pressure_by_state.iter() {
            // if state.pressure_per_minute == cave.max_pressure_per_minute {
            //     panic!("we got max pressure!");
            // }

            // dbg!(&state, &pressure);
            let new_pressure = cave.get_pressure_per_minute(state.valves_open) + pressure;
            let current_valve = &cave.valves[state.location as usize];

            let mut new_states: Vec<State> = Vec::new();
            // If we are in a location with non-zero-rate valve that hasn't been opened yet, try
            // opening it.
            if current_valve.flow_rate != 0 && (state.valves_open & (1 << state.location) == 0) {
                new_states.push(state.open_valve(state.location));
            }
            // Or try moving to any of the exits.
            for exit in current_valve.exits.iter() {
                new_states.push(state.move_to(*exit));
            }

            // Part 2 (only if we have an elephant).
            if use_elephant {
                let elephant_valve = &cave.valves[state.elephant_location as usize];
                let mut new_states2: Vec<State> = Vec::new();
                for new_state in new_states.iter() {
                    // If elephant is in a location with non-zero-rate valve that hasn't been
                    // opened yet, try opening it.
                    if elephant_valve.flow_rate != 0
                        && (new_state.valves_open & (1 << state.elephant_location) == 0)
                    {
                        new_states2.push(new_state.open_valve(state.elephant_location));
                    }
                    // Or try moving the elephant to any of the exits.
                    for exit in elephant_valve.exits.iter() {
                        new_states2.push(new_state.move_elephant_to(*exit));
                    }
                }
                new_states = new_states2;
            }

            new_states.into_iter().for_each(|new_state| {
                next.maybe_update_state(new_state, new_pressure, use_elephant);
            });
        }
        next
    }
}

fn plan(cave: &mut Cave, minutes: usize, use_elephant: bool) -> usize {
    let state_union = StateUnion::new_start(cave.starting_location as u8);
    let mut prev = state_union;
    for i in 0..minutes {
        dbg!(&i, prev.pressure_by_state.len());
        let next = prev.next_minute(cave, use_elephant);
        prev = next;
    }
    *prev.pressure_by_state.values().max().unwrap()
}

fn main() {
    let input = lib::read_input();
    let mut cave = Cave::parse(&input);
    dbg!(&cave);
    // Part 1:
    dbg!(plan(&mut cave, 30, false));
    // Part 2:
    dbg!(plan(&mut cave, 26, true));
}
