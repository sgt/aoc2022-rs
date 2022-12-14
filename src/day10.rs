#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    AddX(i32),
    Noop,
}

impl Op {
    fn cycles(&self) -> u8 {
        match self {
            Op::AddX(_) => 2,
            Op::Noop => 1,
        }
    }
}

#[derive(Debug)]
struct Device {
    program: Vec<Op>,
    reg_x: i32,
    cycle: i32,
    pc: usize,
    cycle_on_op: u8,
}

impl Device {
    fn new(program: &[Op]) -> Self {
        Device {
            program: program.to_vec(),
            reg_x: 1,
            cycle: 0,
            pc: 0,
            cycle_on_op: 0,
        }
    }

    fn op(&self) -> Op {
        self.program[self.pc]
    }

    fn step(&mut self) -> i32 {
        let result = self.reg_x;
        self.cycle += 1;
        self.cycle_on_op += 1;
        if self.cycle_on_op == self.op().cycles() {
            match self.op() {
                Op::Noop => (),
                Op::AddX(n) => self.reg_x += n,
            }
            self.cycle_on_op = 0;
            self.pc += 1;
        }
        result
    }

    fn run(&mut self, cycles: usize) -> i32 {
        // silly 1-off fix
        let mut reg = self.reg_x;
        for _ in 0..cycles {
            reg = self.step();
        }
        self.cycle * reg
    }
}

fn parse(data: &[String]) -> Vec<Op> {
    data.iter()
        .map(|x| {
            let spl: Vec<_> = x.split(' ').collect();
            match spl[0] {
                "addx" => Op::AddX(spl[1].parse().unwrap()),
                "noop" => Op::Noop,
                _ => panic!("unknown command '{}'", spl[0]),
            }
        })
        .collect()
}

pub fn solution1(data: &[String]) -> i32 {
    let program = parse(data);
    let mut device = Device::new(&program);
    let mut result = device.run(20);
    for _ in 0..5 {
        result += device.run(40);
    }
    result
}

pub fn solution2(data: &[String]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        common::str2lines,
        day10::{self, parse, Device},
    };

    fn data() -> Vec<String> {
        str2lines(
            r#"noop
addx 3
addx -5
noop"#,
        )
    }

    fn data2() -> Vec<String> {
        str2lines(
            r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
        )
    }

    #[test]
    fn test_small_example() {
        let program = parse(&data());
        let mut device = Device::new(&program);
        assert_eq!(5 * 4, device.run(5));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(13140, day10::solution1(&data2()));
    }
}
