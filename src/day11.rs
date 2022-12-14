use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum WorryParam {
    Old,
    Num(i32),
}

#[derive(Debug, PartialEq)]
enum WorryOp {
    Add,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    worry_params: (WorryParam, WorryOp, WorryParam),
    /// (divisible_by, if_true, if_false)
    throw_params: (i32, usize, usize),
}

impl Monkey {
    fn parse(data: &[String]) -> (usize, Self) {
        lazy_static! {
            static ref RE_MONKEY: Regex = Regex::new(r"Monkey (\d+):").unwrap();
            static ref RE_ITEMS: Regex = Regex::new(r"Starting items: (.*)$").unwrap();
            static ref RE_OPERATION: Regex =
                Regex::new(r"Operation: new = (\S+) (\S) (\S+)").unwrap();
            static ref RE_TEST: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
            static ref RE_TEST_CLAUSE: Regex =
                Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
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
        let test_true: usize = RE_TEST_CLAUSE.captures(&data[4]).unwrap()[2]
            .parse()
            .unwrap();
        let test_false: usize = RE_TEST_CLAUSE.captures(&data[5]).unwrap()[2]
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
        match s {
            "*" => WorryOp::Mul,
            "+" => WorryOp::Add,
            _ => panic!("unknown op '{}'", s),
        }
    }

    fn parse_worry_param(s: &str) -> WorryParam {
        match s {
            "old" => WorryParam::Old,
            _ => WorryParam::Num(s.parse().unwrap()),
        }
    }

    fn worry_op(&self, old: i32) -> i32 {
        let v1 = match self.worry_params.0 {
            WorryParam::Num(x) => x,
            WorryParam::Old => old,
        };
        let v2 = match self.worry_params.2 {
            WorryParam::Num(x) => x,
            WorryParam::Old => old,
        };
        let result = match self.worry_params.1 {
            WorryOp::Add => v1 + v2,
            WorryOp::Mul => v1 * v2,
        };
        result / 3
    }

    fn throw_op(&self, n: i32) -> usize {
        if n % self.throw_params.0 == 0 {
            self.throw_params.1
        } else {
            self.throw_params.2
        }
    }

    /// Return a list of (monkey_idx, item_to_throw).
    fn turn_results(&self) -> Vec<(usize, i32)> {
        self.items
            .iter()
            .map(|x| self.worry_op(*x))
            .map(|x| (self.throw_op(x), x))
            .collect()
    }
}

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);

impl Monkeys {
    /// Runs a turn and return number of items inspected.
    fn turn(&mut self, idx: usize) -> usize {
        let result = self.0[idx].items.len();
        let tr = self.0[idx].turn_results();
        self.0[idx].items.clear();
        for (i, item) in tr {
            self.0[i].items.push(item);
        }
        result
    }

    /// Return number of items inspected for each monkey.
    fn round(&mut self) -> Vec<usize> {
        (0..self.0.len()).map(|i| self.turn(i)).collect()
    }
}

fn parse(input: &[String]) -> Monkeys {
    let monkey_data: Vec<_> = input.split(|x| x.is_empty()).collect();
    let mut result = Vec::with_capacity(monkey_data.len());
    for md in monkey_data {
        let (idx, monkey) = Monkey::parse(md);
        result.insert(idx, monkey);
    }
    Monkeys(result)
}

pub fn solution1(data: &[String]) -> usize {
    let mut monkeys = parse(data);
    let mut result = vec![0; monkeys.0.len()];

    for _ in 0.. 20{
        result = result
            .iter()
            .zip(monkeys.round())
            .map(|(a, b)| a + b)
            .collect();
    }

    result.sort();
    result.reverse();
    result[0] * result[1]
}

pub fn solution2(data: &[String]) -> usize {
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
