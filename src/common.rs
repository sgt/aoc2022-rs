use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Solution {
    day: u8,
}

impl Solution {
    pub fn new(day: u8) -> Self {
        Solution { day }
    }

    fn input_filename(&self) -> String {
        format!("data/input{}.txt", self.day)
    }

    pub(crate) fn read_input(&self) -> Vec<String> {
        let file = File::open(self.input_filename()).expect("no such file");
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
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
    s.lines().map(|x| x.trim().into()).collect()
}
