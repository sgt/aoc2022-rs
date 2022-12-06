use clap::{command, Parser};
use common::Solution;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// AOC day
    day: u8,
    /// AOC solution part
    part: u8,
}

impl Args {
    pub fn solution(&self) -> Solution {
        Solution::new(self.day)
    }
}

fn main() {
    let args = Args::parse();
    let input = args.solution().read_input();

    // let's not get too clever here
    let solution = match args {
        Args { day: 1, part: 1 } => day1::solution1(&input).to_string(),
        Args { day: 1, part: 2 } => day1::solution2(&input).to_string(),
        Args { day: 2, part: 1 } => day2::solution1(&input).to_string(),
        Args { day: 2, part: 2 } => day2::solution2(&input).to_string(),
        Args { day: 3, part: 1 } => day3::solution1(&input).to_string(),
        Args { day: 3, part: 2 } => day3::solution2(&input).to_string(),
        Args { day: 4, part: 1 } => day4::solution1(&input).to_string(),
        Args { day: 4, part: 2 } => day4::solution2(&input).to_string(),
        Args { day: 5, part: 1 } => day5::solution1(&input),
        Args { day: 5, part: 2 } => day5::solution2(&input),
        Args { day: 6, part: 1 } => day6::solution1(&input).to_string(),
        Args { day: 6, part: 2 } => day6::solution2(&input).to_string(),
        _ => unimplemented!("this solution does not exist yet"),
    };
    println!("{}", solution);
}
