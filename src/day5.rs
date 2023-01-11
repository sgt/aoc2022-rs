use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Self {
            amount: caps[1].parse().unwrap(),
            from: caps[2].parse().unwrap(),
            to: caps[3].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Crates(Vec<Vec<char>>);

impl Crates {
    fn parse(data: &[String]) -> Self {
        let mut iter = data.iter().rev();
        let ncol = iter.next().unwrap().split_whitespace().count();
        let mut crates = Self(Vec::new());

        for _ in 0..ncol {
            crates.0.push(Vec::new());
        }

        for s in iter {
            for (i, c) in s.as_bytes().chunks(4).enumerate() {
                let letter = c[1] as char;
                if letter != ' ' {
                    crates.0[i].push(letter);
                }
            }
        }

        crates
    }

    fn execute(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.amount {
            let c = self.0[instruction.from - 1].pop().unwrap();
            self.0[instruction.to - 1].push(c);
        }
    }

    fn execute2(&mut self, instruction: &Instruction) {
        let from = &mut self.0[instruction.from - 1];
        let mut cs = from.drain(from.len() - instruction.amount..).collect();
        self.0[instruction.to - 1].append(&mut cs);
    }

    fn top_message(&self) -> String {
        self.0.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn parse_input(data: &[String]) -> (Crates, Vec<Instruction>) {
    let chunks: Vec<_> = data.split(String::is_empty).collect();
    (
        Crates::parse(chunks[0]),
        chunks[1].iter().map(|x| Instruction::parse(x)).collect(),
    )
}

pub fn solution1(data: &[String]) -> String {
    let (mut crates, instructions) = parse_input(data);
    for i in instructions {
        crates.execute(&i);
    }
    crates.top_message()
}

pub fn solution2(data: &[String]) -> String {
    let (mut crates, instructions) = parse_input(data);
    for i in instructions {
        crates.execute2(&i);
    }
    crates.top_message()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day5};

    fn data() -> Vec<String> {
        str2lines(
            r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!("CMZ", day5::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!("MCD", day5::solution2(&data()));
    }
}
