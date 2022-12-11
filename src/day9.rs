use std::collections::HashSet;

enum Axis {
    X,
    Y,
}

struct Command(Axis, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords(i32, i32);

impl Coords {
    fn step(&mut self, cmd: &Command) {
        match cmd.0 {
            Axis::X => self.0 += cmd.1.signum(),
            Axis::Y => self.1 += cmd.1.signum(),
        }
    }

    fn move_tail(&mut self, head: &Coords) {
        let x_diff = head.0 - self.0;
        let y_diff = head.1 - self.1;
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            return;
        }
        self.0 += x_diff.signum();
        self.1 += y_diff.signum();
    }
}

fn parse(data: &[String]) -> Vec<Command> {
    data.iter()
        .map(|x| {
            let spl: Vec<_> = x.split(' ').collect();
            let n = spl[1].parse().unwrap();
            match spl[0] {
                "U" => Command(Axis::Y, n),
                "D" => Command(Axis::Y, -n),
                "L" => Command(Axis::X, -n),
                "R" => Command(Axis::X, n),
                _ => panic!("unknown command '{}'", spl[0]),
            }
        })
        .collect()
}

fn tail_visited(commands: &[Command]) -> HashSet<Coords> {
    let mut h = Coords(0, 0);
    let mut t = Coords(0, 0);
    let mut result = HashSet::from_iter(vec![t]);
    for cmd in commands {
        // moving head all the way and then tracing step by step also works
        // but let's do both step by step anyway
        for _ in 0..cmd.1.abs() {
            h.step(cmd);
            t.move_tail(&h);
            result.insert(t);
        }
    }
    result
}

fn draw(data: &HashSet<Coords>) -> Vec<String> {
    let mut result = vec![];
    let min_x = data.iter().map(|x| x.0).min().unwrap();
    let min_y = data.iter().map(|x| x.1).min().unwrap();
    let max_x = data.iter().map(|x| x.0).max().unwrap();
    let max_y = data.iter().map(|x| x.1).max().unwrap();
    for y in (min_y..=max_y).rev() {
        let mut line = String::new();
        for x in min_x..=max_x {
            let c = if data.contains(&Coords(x, y)) {
                '#'
            } else {
                '.'
            };
            line.push(c);
        }
        result.push(line);
    }
    result
}

pub fn solution1(data: &[String]) -> usize {
    let commands = parse(data);
    let data = tail_visited(&commands);
    data.len()
}

pub fn solution2(data: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day9};

    fn data() -> Vec<String> {
        str2lines(
            r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
        )
    }

    fn data2() -> Vec<String> {
        str2lines(
            r#"D 20
R 20
U 20
R 2"#,
        )
    }
    #[test]
    fn test_solution1() {
        assert_eq!(13, day9::solution1(&data()));
        assert_eq!(59, day9::solution1(&data2()));
    }
}
