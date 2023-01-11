use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Network<T> {
    edges: HashSet<(T, T)>,
    valves: HashMap<T, u32>,
}

#[derive(Debug, Default)]
struct Path<T> {
    opened_valves: HashSet<T>,
    released_pressure: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Default, Copy, Debug)]
struct Name(u16);

impl From<&str> for Name {
    fn from(item: &str) -> Self {
        let bytes = item.as_bytes();
        Name(u16::from(bytes[0]) * 256 + u16::from(bytes[1]))
    }
}

impl<T: PartialOrd + Eq + Hash + Copy + Default> Network<T> {
    pub fn new(edges: HashSet<(T, T)>, pressures: HashMap<T, u32>) -> Self {
        // not interested in zero pressures
        let pressures: HashMap<_, _> = pressures.into_iter().filter(|(_, v)| *v > 0).collect();
        Self {
            edges,
            valves: pressures,
        }
    }

    /// Floyd-Warshall
    fn shortest_paths(&self) -> HashMap<(T, T), u32> {
        let mut dist: HashMap<(T, T), u32> = self
            .edges
            .iter()
            .flat_map(|&(a, b)| [((a, b), 1), ((a, a), 0), ((b, b), 0)])
            .collect();

        for &k in self.valves.keys() {
            for &i in self.valves.keys() {
                for &j in self.valves.keys() {
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
        dist.into_iter().filter(|&(_, v)| v != 0).collect()
    }

    #[inline]
    fn norm(a: T, b: T) -> (T, T) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    fn all_paths(&self, minutes: u32) -> Vec<Path<T>> {
        fn rec<T>(path_so_far: Path<T>) -> Vec<Path<T>> {
            let vertices_to_explore = todo!();
            todo!()
        }
        rec(Path::default())
    }

    pub fn path_with_max_pressure(&self, minutes: u32) -> u32 {
        self.all_paths(minutes)
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
            edges.insert(Network::norm(name, neighbour));
        }
        if pressure > 0 {
            pressures.insert(name, pressure);
        }
    }

    Network::new(edges, pressures)
}

pub fn solution1(input: &[String]) -> u32 {
    parse(input).path_with_max_pressure(30)
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
    #[ignore]
    fn test_solution1() {
        assert_eq!(1651, day16::solution1(&data()));
    }

    #[test]
    fn test_floyd_warshall() {
        let edges = [('a', 'b'), ('a', 'c'), ('b', 'd'), ('c', 'd')];
        let network = Network::new(
            edges.into(),
            [('a', 1), ('b', 1), ('c', 1), ('d', 1)].into(),
        );
        let result = network.shortest_paths();
        assert_eq!(
            sorted(result).collect::<Vec<_>>(),
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
