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

    fn move_tail(&mut self, head: Coords) {
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
            let (cmd, n) = x.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match cmd {
                "U" => Command(Axis::Y, n),
                "D" => Command(Axis::Y, -n),
                "L" => Command(Axis::X, -n),
                "R" => Command(Axis::X, n),
                _ => panic!("unknown command '{cmd}'"),
            }
        })
        .collect()
}

fn tail_visited<const N: usize>(commands: &[Command]) -> HashSet<Coords> {
    let mut rope = [Coords(0, 0); N];
    let mut result = HashSet::new();
    for cmd in commands {
        // moving head all the way and then tracing step by step also works
        // but let's do both step by step anyway
        for _ in 0..cmd.1.abs() {
            for i in 0..rope.len() {
                if i == 0 {
                    rope[i].step(cmd);
                } else {
                    let prev = rope[i - 1];
                    rope[i].move_tail(prev);
                }
                if i == rope.len() - 1 {
                    result.insert(rope[i]);
                }
            }
        }
    }
    result
}

#[allow(dead_code)]
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
    let data = tail_visited::<2>(&commands);
    data.len()
}

pub fn solution2(data: &[String]) -> usize {
    let commands = parse(data);
    let data = tail_visited::<10>(&commands);
    data.len()
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
            r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(13, day9::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(1, day9::solution2(&data()));
        assert_eq!(36, day9::solution2(&data2()));
    }
}
