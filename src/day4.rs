use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

struct Assignment {
    e1: RangeInclusive<i32>,
    e2: RangeInclusive<i32>,
}

impl Assignment {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Self {
            e1: caps[1].parse().unwrap()..=caps[2].parse().unwrap(),
            e2: caps[3].parse().unwrap()..=caps[4].parse().unwrap(),
        }
    }

    fn has_full_overlap(&self) -> bool {
        (self.e1.start() <= self.e2.start() && self.e1.end() >= self.e2.end())
            || (self.e2.start() <= self.e1.start() && self.e2.end() >= self.e1.end())
    }

    fn has_overlap(&self) -> bool {
        (self.e1.start() <= self.e2.start() && self.e1.end() >= self.e2.start())
            || (self.e2.start() <= self.e1.start() && self.e2.end() >= self.e1.start())
    }
}

pub fn solution1(data: &[String]) -> i32 {
    data.iter()
        .map(|x| Assignment::parse(x))
        .filter(|x| x.has_full_overlap())
        .count() as i32
}

pub fn solution2(data: &[String]) -> i32 {
    data.iter()
        .map(|x| Assignment::parse(x))
        .filter(|x| x.has_overlap())
        .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day4};

    fn data() -> Vec<String> {
        str2lines(
            r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(2, day4::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(4, day4::solution2(&data()));
    }
}
