#![allow(clippy::range_plus_one)]
use std::ops::{Bound, Range, RangeBounds};

use lazy_static::lazy_static;
use ranges::Ranges;
use regex::Regex;

type Pos = (i32, i32);

struct Sensor {
    pos: Pos,
    beacon: Pos,
    distance: u32,
}

impl Sensor {
    fn new(pos: Pos, beacon: Pos) -> Self {
        Self {
            pos,
            beacon,
            distance: distance(pos, beacon),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[inline]
    fn covered_x(&self, y: i32) -> Range<i32> {
        let y_dist = y.abs_diff(self.pos.1);
        if y_dist > self.distance {
            0..0
        } else {
            let offset = self.distance - y_dist;
            (self.pos.0 - offset as i32)..(self.pos.0 + offset as i32 + 1)
        }
    }
}

struct Grid {
    sensors: Vec<Sensor>,
}

impl Grid {
    #[inline]
    fn covered_x(&self, y: i32) -> Ranges<i32> {
        let mut ranges: Ranges<i32> = Ranges::new();
        for s in &self.sensors {
            ranges.insert(s.covered_x(y));
        }
        ranges
    }

    fn covered_x_sans_beacons(&self, y: i32) -> Ranges<i32> {
        let mut ranges = self.covered_x(y);
        for s in &self.sensors {
            if s.beacon.1 == y {
                ranges.remove(s.beacon.1..s.beacon.1 + 1);
            }
        }
        ranges
    }

    #[inline]
    fn uncovered(&self, y: i32, boundary: Range<i32>) -> Ranges<i32> {
        let mut solution: Ranges<i32> = Ranges::new();
        solution.insert(boundary);
        for r in self.covered_x(y).as_slice() {
            solution.remove(*r);
        }
        solution
    }
}

fn distance(p1: Pos, p2: Pos) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn parse(input: &[String]) -> Grid {
    lazy_static! {
        static ref RE_INPUT: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    let mut sensors = vec![];
    for line in input {
        if let Some(cap) = RE_INPUT.captures(line) {
            sensors.push(Sensor::new(
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            ));
        } else {
            panic!("cannot parse line '{line}'");
        }
    }
    Grid { sensors }
}

fn ranges_element_count(rr: &Ranges<i32>) -> usize {
    rr.as_slice().iter().map(|x| x.into_iter().count()).sum()
}

pub fn solve1(input: &[String], line: i32) -> usize {
    let ranges = parse(input).covered_x_sans_beacons(line);
    ranges_element_count(&ranges)
}

pub fn solve2(input: &[String], boundary: Range<i32>) -> Option<i64> {
    let grid = parse(input);

    for y in boundary.clone() {
        if y % 20000 == 0 {
            eprintln!("{y}");
        }
        let x = grid.uncovered(y, boundary.clone());
        if x.len() > 1 {
            return None;
        }
        if x.len() == 1 {
            let r = x.as_slice()[0];
            if r.is_singleton() {
                return if let Bound::Included(&v) = r.start_bound() {
                    Some(i64::from(v) * 4_000_000 + i64::from(y))
                } else {
                    None
                };
            }
            return None;
        }
    }
    None
}

pub fn solution1(input: &[String]) -> usize {
    solve1(input, 2_000_000)
}

pub fn solution2(input: &[String]) -> i64 {
    solve2(input, 0..4_000_001).unwrap()
}

#[cfg(test)]
mod tests {
    use ranges::Ranges;

    use crate::{common::str2lines, day15};

    #[test]
    fn test_ranges() {
        let mut ranges = Ranges::new();
        ranges.insert(0..3);
        ranges.insert(2..5);
        ranges.insert(7..9);
        ranges.remove(4..8);
        assert_eq!(5, day15::ranges_element_count(&ranges));
    }

    #[test]
    fn test_sensor() {
        let sensor = day15::Sensor::new((8, 7), (2, 10));
        assert_eq!(sensor.covered_x(-2), 8..9);
        assert_eq!(sensor.covered_x(-1), 7..10);
    }

    fn data() -> Vec<String> {
        str2lines(
            r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(26, day15::solve1(&data(), 10));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(56_000_011, day15::solve2(&data(), 0..21).unwrap());
    }
}
