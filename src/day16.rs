use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Default, Copy)]
struct Name(u16);

impl From<&str> for Name {
    fn from(item: &str) -> Self {
        let bytes = item.as_bytes();
        Name(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0.to_le_bytes();
        write!(f, "{}{}", bytes[0] as char, bytes[1] as char)
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug)]
struct Network<T> {
    valves: HashMap<T, u32>,
    nonzero_valves: HashSet<T>,
    shortest_paths: HashMap<(T, T), u32>,
}

#[derive(Debug, Default, Clone)]
struct Path<T> {
    opened_valves: HashSet<T>,
    pressure_per_minute: u32,
    current_valve: T,
    released_pressure: u32,
    minutes_passed: u32,
}

impl<T: PartialOrd + Eq + Hash + Copy + Display + std::fmt::Debug + Default> Network<T> {
    pub fn new(edges: &HashSet<(T, T)>, pressures: HashMap<T, u32>) -> Self {
        Self {
            nonzero_valves: Self::nonzero_valves(&pressures),
            shortest_paths: Self::shortest_paths(&pressures, edges),
            valves: pressures,
        }
    }

    fn nonzero_valves(valves: &HashMap<T, u32>) -> HashSet<T> {
        valves
            .iter()
            .filter_map(|(&edge, &value)| (value != 0).then_some(edge))
            .collect()
    }

    /// Floyd-Warshall
    fn shortest_paths(valves: &HashMap<T, u32>, edges: &HashSet<(T, T)>) -> HashMap<(T, T), u32> {
        let mut dist: HashMap<(T, T), u32> = edges
            .iter()
            .flat_map(|&(a, b)| [((a, b), 1), ((a, a), 0), ((b, b), 0)])
            .collect();

        let vv = valves.keys();
        for &k in vv.clone() {
            for &i in vv.clone() {
                for &j in vv.clone() {
                    let ij = dist.get(&(i, j));
                    let ik = dist.get(&(i, k));
                    let kj = dist.get(&(k, j));
                    if let (Some(&ikv), Some(&kjv)) = (ik, kj) {
                        if ij.is_none() || *ij.unwrap() > ikv + kjv {
                            dist.insert((i, j), ikv + kjv);
                        }
                    }
                }
            }
        }
        dist.into_iter().collect()
    }

    fn all_paths_from(&self, path_so_far: Path<T>, minutes: u32) -> Vec<Path<T>> {
        let valves_to_explore: HashSet<_> = self
            .nonzero_valves
            .difference(&path_so_far.opened_valves)
            .copied()
            .filter(|&v| {
                if let Some(&segment_length) =
                    self.shortest_paths.get(&(path_so_far.current_valve, v))
                {
                    minutes> segment_length + 1
                } else {
                    false
                }
            })
            .collect();
        if valves_to_explore.is_empty() {
            // can't visit more valves
            vec![Path {
                released_pressure: path_so_far.released_pressure
                    + path_so_far.pressure_per_minute * minutes,
                ..path_so_far
            }]
        } else {
            valves_to_explore
                .iter()
                .flat_map(|&next_valve| {
                    let minutes_added =
                        self.shortest_paths[&(path_so_far.current_valve, next_valve)] + 1;
                    let next_pressure = self.valves[&next_valve];
                    let mut next_path = path_so_far.clone();
                    next_path.opened_valves.insert(next_valve);
                    next_path.pressure_per_minute += next_pressure;
                    next_path.current_valve = next_valve;
                    next_path.released_pressure += path_so_far.pressure_per_minute * minutes_added;
                    next_path.minutes_passed += minutes_added;

                    self.all_paths_from(next_path, minutes- minutes_added)
                })
                .collect()
        }
    }

    pub fn path_with_max_pressure(&self, start: T, minutes: u32) -> u32 {
        let init_path = Path {
            current_valve: start,
            ..Path::default()
        };
        self.all_paths_from(init_path, minutes)
            .iter()
            .map(|p| p.released_pressure)
            .max()
            .unwrap()
    }
}

fn parse(input: &[String]) -> Network<Name> {
    lazy_static! {
        static ref RE_INPUT: Regex =
            Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
                .unwrap();
    }

    let mut pressures = HashMap::new();
    let mut edges = HashSet::new();

    for line in input {
        let cap = RE_INPUT.captures(line).unwrap();
        let name: Name = cap[1].into();
        let pressure: u32 = cap[2].parse().unwrap();
        let neighbours: Vec<Name> = cap[3].split(", ").map(Into::into).collect();
        for neighbour in neighbours {
            edges.insert((name, neighbour));
        }
        pressures.insert(name, pressure);
    }

    Network::new(&edges, pressures)
}

pub fn solution1(input: &[String]) -> u32 {
    parse(input).path_with_max_pressure("AA".into(), 30)
}

pub fn solution2(input: &[String]) -> u32 {
    parse(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day16};
    use itertools::sorted;
    use pretty_assertions::assert_eq;

    use super::Network;

    fn data() -> Vec<String> {
        str2lines(
            r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(1651, day16::solution1(&data()));
    }

    #[test]
    fn test_floyd_warshall() {
        let edges = [('a', 'b'), ('a', 'c'), ('b', 'd'), ('c', 'd')];
        let network = Network::new(
            &edges.into(),
            [('a', 1), ('b', 1), ('c', 1), ('d', 1)].into(),
        );
        let result = Network::shortest_paths(&network.valves, &edges.into());
        assert_eq!(
            sorted(result).filter(|&(_, v)| v != 0).collect::<Vec<_>>(),
            sorted(vec![
                (('a', 'b'), 1),
                (('a', 'c'), 1),
                (('b', 'd'), 1),
                (('c', 'd'), 1),
                (('a', 'd'), 2)
            ])
            .collect::<Vec<_>>()
        );
    }
}
