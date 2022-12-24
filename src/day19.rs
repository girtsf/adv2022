mod lib;

use std::collections::VecDeque;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    orerob_cost_ore: usize,
    clayrob_cost_ore: usize,
    obsrob_cost_ore: usize,
    obsrob_cost_clay: usize,
    georob_cost_ore: usize,
    georob_cost_obs: usize,
}

#[derive(Debug, Default, Clone)]
struct State {
    minute: usize,
    ore: usize,
    clay: usize,
    obs: usize,
    geo: usize,
    ore_robots: usize,
    clay_robots: usize,
    obs_robots: usize,
    geo_robots: usize,
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        lazy_static! {
            // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian
            // robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            static ref RE: Regex =
                Regex::new(r"(?x)^
                    Blueprint\s+(?P<id>\d+):.*
                    ore\ robot\ costs\ (?P<orerob_cost_ore>\d+)\ ore.*
                    clay\ robot\ costs\ (?P<clayrob_cost_ore>\d+)\ ore.*
                    obsidian\ robot\ costs\ (?P<obsrob_cost_ore>\d+)\ ore\ and\ (?P<obsrob_cost_clay>\d+)\ clay.*
                    geode\ robot\ costs\ (?P<georob_cost_ore>\d+)\ ore\ and\ (?P<georob_cost_obs>\d+)\ obsidian
                    \.$ ").unwrap();
        }
        let c = RE.captures(line).unwrap();
        let extract = |name: &str| c.name(name).unwrap().as_str().parse().unwrap();
        Blueprint {
            id: extract("id"),
            orerob_cost_ore: extract("orerob_cost_ore"),
            clayrob_cost_ore: extract("clayrob_cost_ore"),
            obsrob_cost_ore: extract("obsrob_cost_ore"),
            obsrob_cost_clay: extract("obsrob_cost_clay"),
            georob_cost_ore: extract("georob_cost_ore"),
            georob_cost_obs: extract("georob_cost_obs"),
        }
    }

    fn find_max_geodes(&self, minutes: usize) -> usize {
        let mut todo: VecDeque<State> = VecDeque::from([State::new()]);
        let mut best = 0usize;

        while let Some(state) = todo.pop_front() {
            if state.minute > (minutes + 1) {
                continue;
            }
            let remaining_minutes = minutes + 1 - state.minute;
            let geodes = remaining_minutes * state.geo_robots + state.geo;
            if geodes > best {
                best = geodes;
                // dbg!(&state.minute, &best);
            }
            if remaining_minutes == 0 {
                continue;
            }
            // Conservatively in the best possible case, we keep building one geode robot each
            // minute, and those produce geodes. If that can't beat our current best score, don't
            // bother.
            let best_possible = state.geo
                + state.geo_robots * remaining_minutes
                + (remaining_minutes - 1) * remaining_minutes / 2;
            if best_possible <= best {
                // dbg!(&state.minute, &best, &best_possible);
                continue;
            }

            // dbg!(&state);
            let next = state.get_next_states(&self);
            // dbg!(&next);
            todo.extend(next);
        }
        best
    }
}

fn div_round_up(a: usize, b: usize) -> usize {
    (a + (b - 1)) / b
}

impl State {
    fn new() -> State {
        State {
            minute: 1,
            ore_robots: 1,
            ..State::default()
        }
    }

    fn get_next_states(&self, bp: &Blueprint) -> Vec<State> {
        let mut out = Vec::new();
        // Build an ore robot next.
        {
            let elapsed = 1 + if self.ore >= bp.orerob_cost_ore {
                0
            } else {
                div_round_up(bp.orerob_cost_ore - self.ore, self.ore_robots)
            };
            out.push(State {
                minute: self.minute + elapsed,
                ore: self.ore + self.ore_robots * elapsed - bp.orerob_cost_ore,
                clay: self.clay + self.clay_robots * elapsed,
                obs: self.obs + self.obs_robots * elapsed,
                geo: self.geo + self.geo_robots * elapsed,
                ore_robots: self.ore_robots + 1,
                ..*self
            });
        }

        // Build a clay robot next.
        {
            let elapsed = 1 + if self.ore >= bp.clayrob_cost_ore {
                0
            } else {
                div_round_up(bp.clayrob_cost_ore - self.ore, self.ore_robots)
            };
            out.push(State {
                minute: self.minute + elapsed,
                ore: self.ore + self.ore_robots * elapsed - bp.clayrob_cost_ore,
                clay: self.clay + self.clay_robots * elapsed,
                obs: self.obs + self.obs_robots * elapsed,
                geo: self.geo + self.geo_robots * elapsed,
                clay_robots: self.clay_robots + 1,
                ..*self
            });
        }

        // Build an obsidian robot next.
        //
        // We must have at least one clay robot, otherwise, don't bother.
        if self.clay_robots > 0 {
            let elapsed = 1
                + (if self.ore >= bp.obsrob_cost_ore {
                    0
                } else {
                    div_round_up(bp.obsrob_cost_ore - self.ore, self.ore_robots)
                })
                .max(if self.clay >= bp.obsrob_cost_clay {
                    0
                } else {
                    div_round_up(bp.obsrob_cost_clay - self.clay, self.clay_robots)
                });
            out.push(State {
                minute: self.minute + elapsed,
                ore: self.ore + self.ore_robots * elapsed - bp.obsrob_cost_ore,
                clay: self.clay + self.clay_robots * elapsed - bp.obsrob_cost_clay,
                obs: self.obs + self.obs_robots * elapsed,
                geo: self.geo + self.geo_robots * elapsed,
                obs_robots: self.obs_robots + 1,
                ..*self
            });
        }

        // Build a geode cracking robot next.
        //
        // We must have at least one obsidian robot, otherwise, don't bother.
        if self.obs_robots > 0 {
            let elapsed = 1
                + (if self.ore >= bp.georob_cost_ore {
                    0
                } else {
                    div_round_up(bp.georob_cost_ore - self.ore, self.ore_robots)
                })
                .max(if self.obs >= bp.georob_cost_obs {
                    0
                } else {
                    div_round_up(bp.georob_cost_obs - self.obs, self.obs_robots)
                });
            out.push(State {
                minute: self.minute + elapsed,
                ore: self.ore + self.ore_robots * elapsed - bp.georob_cost_ore,
                clay: self.clay + self.clay_robots * elapsed,
                obs: self.obs + self.obs_robots * elapsed - bp.georob_cost_obs,
                geo: self.geo + self.geo_robots * elapsed,
                geo_robots: self.geo_robots + 1,
                ..*self
            });
        }
        out
    }
}

fn main() {
    let input = lib::read_input();
    let blueprints = input
        .lines()
        .map(|line| Blueprint::parse(line))
        .collect_vec();
    dbg!(&blueprints);
    let part1 = blueprints.iter().map(|b| b.find_max_geodes(24) * b.id).sum::<usize>();
    dbg!(&part1);
    // dbg!(blueprints[1].find_max_geodes(32));
    let part2 = blueprints[0..3].iter().fold(1, |acc, b| acc * b.find_max_geodes(32));
    dbg!(&part2);
}
