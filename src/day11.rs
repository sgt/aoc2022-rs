use lazy_static::lazy_static;
use regex::Regex;

enum WorryParam {
    Old,
    Num(i32),
}
enum WorryOp {
    Add,
    Mul,
}

struct Monkey {
    items: Vec<i32>,
    worry_params: (WorryParam, WorryOp, WorryParam),
    throw_params: (i32, i32, i32),
}

impl Monkey {
    fn parse(data: &[String]) -> (usize, Self) {
        lazy_static! {
            static ref RE_MONKEY: Regex = Regex::new(r"Monkey (\d+):").unwrap();
            static ref RE_ITEMS: Regex = Regex::new(r"Starting items: (.*)$").unwrap();
            static ref RE_OPERATION: Regex = Regex::new(r"Operation: (\S+) (\S) (\S+)").unwrap();
            static ref RE_TEST: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
            static ref RE_TEST_CLAUSE: Regex =
                Regex::new(r"If [true|false]: throw to monkey (\d+)").unwrap();
        }
        let idx: usize = RE_MONKEY.captures(&data[0]).unwrap()[1].parse().unwrap();
        let items: Vec<i32> = RE_ITEMS.captures(&data[1]).unwrap()[1]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let op_caps = RE_OPERATION.captures(&data[2]).unwrap();
        let worry_params = (
            Self::parse_worry_param(&op_caps[1]),
            Self::parse_worry_op(&op_caps[2]),
            Self::parse_worry_param(&op_caps[3]),
        );
        let test_param: i32 = RE_TEST.captures(&data[3]).unwrap()[1].parse().unwrap();
        let test_true: i32 = RE_TEST_CLAUSE.captures(&data[4]).unwrap()[1]
            .parse()
            .unwrap();
        let test_false: i32 = RE_TEST_CLAUSE.captures(&data[5]).unwrap()[1]
            .parse()
            .unwrap();
        (
            idx,
            Monkey {
                items,
                worry_params,
                throw_params: (test_param, test_true, test_false),
            },
        )
    }

    fn parse_worry_op(s: &str) -> WorryOp {
        if s == "*" {
            WorryOp::Mul
        } else {
            WorryOp::Add
        }
    }

    fn parse_worry_param(s: &str) -> WorryParam {
        if s == "old" {
            WorryParam::Old
        } else {
            WorryParam::Num(s.parse().unwrap())
        }
    }

    fn worry_op(n: i32) -> i32 {
        todo!()
    }

    fn throw_op(n: i32) -> i32 {
        todo!()
    }

    fn turn(&mut self) {
        todo!()
    }
}

fn parse(input: &[String]) -> Vec<Monkey> {
    let monkey_data: Vec<_> = input.split(|x| x.is_empty()).collect();
    let mut result = Vec::with_capacity(monkey_data.len());
    for md in monkey_data {
        let (idx, monkey) = Monkey::parse(md);
        result[idx] = monkey;
    }
    result
}

pub fn solution1(data: &[String]) -> i32 {
    let input = parse(data);
    todo!()
}

pub fn solution2(data: &[String]) -> i32 {
    let input = parse(data);
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day11};

    fn data() -> Vec<String> {
        str2lines(
            r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
        )
    }
    #[test]
    fn test_solution1() {
        assert_eq!(10605, day11::solution1(&data()));
    }
}
