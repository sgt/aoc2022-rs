use gcd::Gcd;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum WorryParam {
    Old,
    Num(u64),
}

#[derive(Debug, PartialEq)]
enum WorryOp {
    Add,
    Mul,
}

#[derive(Debug)]
struct ThrowPlan {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey<const V: u64> {
    items: Vec<u64>,
    worry_plan: (WorryParam, WorryOp, WorryParam),
    /// (divisible_by, if_true, if_false)
    throw_plan: ThrowPlan,
}

impl<const V: u64> Monkey<V> {
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
        let items: Vec<u64> = RE_ITEMS.captures(&data[1]).unwrap()[1]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let op_caps = RE_OPERATION.captures(&data[2]).unwrap();
        let worry_params = (
            Self::parse_worry_param(&op_caps[1]),
            Self::parse_worry_op(&op_caps[2]),
            Self::parse_worry_param(&op_caps[3]),
        );
        let divisible_by: u64 = RE_TEST.captures(&data[3]).unwrap()[1].parse().unwrap();
        let if_true: usize = RE_TEST_CLAUSE.captures(&data[4]).unwrap()[2]
            .parse()
            .unwrap();
        let if_false: usize = RE_TEST_CLAUSE.captures(&data[5]).unwrap()[2]
            .parse()
            .unwrap();
        (
            idx,
            Monkey {
                items,
                worry_plan: worry_params,
                throw_plan: ThrowPlan {
                    divisor: divisible_by,
                    if_true,
                    if_false,
                },
            },
        )
    }

    fn parse_worry_op(s: &str) -> WorryOp {
        match s {
            "*" => WorryOp::Mul,
            "+" => WorryOp::Add,
            _ => panic!("unknown op '{s}'"),
        }
    }

    fn parse_worry_param(s: &str) -> WorryParam {
        match s {
            "old" => WorryParam::Old,
            _ => WorryParam::Num(s.parse().unwrap()),
        }
    }

    fn worry_op(&self, old: u64) -> u64 {
        let v1 = match self.worry_plan.0 {
            WorryParam::Num(x) => x,
            WorryParam::Old => old,
        };
        let v2 = match self.worry_plan.2 {
            WorryParam::Num(x) => x,
            WorryParam::Old => old,
        };
        let result = match self.worry_plan.1 {
            WorryOp::Add => v1 + v2,
            WorryOp::Mul => v1 * v2,
        };
        match V {
            1 => result / 3,
            2 => result,
            _ => panic!("unknown version"),
        }
    }

    fn throw_op(&self, n: u64) -> usize {
        if n % self.throw_plan.divisor == 0 {
            self.throw_plan.if_true
        } else {
            self.throw_plan.if_false
        }
    }

    /// Return a list of (`monkey_idx`, `item_to_throw`).
    fn turn_results(&self, divisors_lcm: u64) -> Vec<(usize, u64)> {
        self.items
            .iter()
            .map(|x| self.worry_op(*x % divisors_lcm))
            .map(|x| (self.throw_op(x), x))
            .collect()
    }
}

#[derive(Debug)]
struct Monkeys<const V: u64> {
    v: Vec<Monkey<V>>,
    divisors_lcm: u64,
}

impl<const V: u64> Monkeys<V> {
    fn new(v: Vec<Monkey<V>>) -> Self {
        let divisors_lcm = v
            .iter()
            .map(|x| x.throw_plan.divisor)
            .reduce(|acc, x| (acc * x) / acc.gcd(x))
            .unwrap();
        Self { v, divisors_lcm }
    }

    /// Runs a turn and return number of items inspected.
    fn turn(&mut self, idx: usize) -> usize {
        let result = self.v[idx].items.len();
        let tr = self.v[idx].turn_results(self.divisors_lcm);
        self.v[idx].items.clear();
        for (i, item) in tr {
            self.v[i].items.push(item);
        }
        result
    }

    /// Return number of items inspected for each monkey.
    fn round(&mut self) -> Vec<usize> {
        (0..self.v.len()).map(|i| self.turn(i)).collect()
    }
}

fn parse<const V: u64>(input: &[String]) -> Monkeys<V> {
    let monkey_data: Vec<_> = input.split(String::is_empty).collect();
    let mut result = Vec::with_capacity(monkey_data.len());
    for md in monkey_data {
        let (idx, monkey) = Monkey::parse(md);
        result.insert(idx, monkey);
    }
    Monkeys::new(result)
}

pub fn solution<const V: u64>(data: &[String], rounds: usize) -> usize {
    let mut monkeys: Monkeys<V> = parse(data);
    let mut result = vec![0; monkeys.v.len()];

    for _ in 0..rounds {
        result = result
            .iter()
            .zip(monkeys.round())
            .map(|(a, b)| a + b)
            .collect();
    }

    result.sort_unstable();
    result.reverse();
    result[0] * result[1]
}

pub fn solution1(data: &[String]) -> usize {
    solution::<1>(data, 20)
}

pub fn solution2(data: &[String]) -> usize {
    solution::<2>(data, 10_000)
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

    #[test]
    fn test_solution2() {
        assert_eq!(2_713_310_158, day11::solution2(&data()));
    }
}
