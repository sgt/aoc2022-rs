use clap::{command, Parser};
use solution::Solution;

mod common;
mod solution;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
        Solution::new(self.day, self.part)
    }
}

fn main() {
    let args = Args::parse();
    println!("{}", args.solution().solve());
}
