use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day1;
use crate::day2;
use crate::day3;
use crate::day4;
use crate::day5;
use crate::day6;

pub struct Solution {
    day: u8,
    part: u8,
}

impl Solution {
    pub fn new(day: u8, part: u8) -> Self {
        Solution { day, part }
    }

    fn input_filename(&self) -> String {
        format!("data/input{}.txt", self.day)
    }

    fn read_input(&self) -> Vec<String> {
        let file = File::open(self.input_filename()).expect("no such file");
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
    }

    pub fn solve(&self) -> String {
        let input = self.read_input();

        // let's not get too clever here
        match self {
            Self { day: 1, part: 1 } => day1::solution1(&input).to_string(),
            Self { day: 1, part: 2 } => day1::solution2(&input).to_string(),
            Self { day: 2, part: 1 } => day2::solution1(&input).to_string(),
            Self { day: 2, part: 2 } => day2::solution2(&input).to_string(),
            Self { day: 3, part: 1 } => day3::solution1(&input).to_string(),
            Self { day: 3, part: 2 } => day3::solution2(&input).to_string(),
            Self { day: 4, part: 1 } => day4::solution1(&input).to_string(),
            Self { day: 4, part: 2 } => day4::solution2(&input).to_string(),
            Self { day: 5, part: 1 } => day5::solution1(&input),
            Self { day: 5, part: 2 } => day5::solution2(&input),
            Self { day: 6, part: 1 } => day6::solution1(&input).to_string(),
            Self { day: 6, part: 2 } => day6::solution2(&input).to_string(),
            _ => unimplemented!("this solution does not exist yet"),
        }
    }
}

fn vec_of_nums(v: &[String]) -> Vec<i32> {
    v.iter()
        .map(|x| x.parse().expect("invalid integer"))
        .collect()
}

pub(crate) fn int_groups_data(data: &[String]) -> Vec<Vec<i32>> {
    data.split(|v| v.is_empty()).map(vec_of_nums).collect()
}

#[cfg(test)]
pub(crate) fn str2lines(s: &str) -> Vec<String> {
    s.lines().map(|x| x.into()).collect()
}
