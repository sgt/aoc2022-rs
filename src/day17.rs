use lazy_static::lazy_static;
use std::io::{self,Write};
use std::{cmp::max, collections::HashSet, fmt::Display};

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn shift(self) -> isize {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Shape(Vec<Pos>);

impl Shape {
    fn push(&self, dir: Dir) -> Shape {
        Shape(self.0.iter().map(|&(x, y)| (x + dir.shift(), y)).collect())
    }

    fn pushed(&self, dir: Dir, n: usize) -> Self {
        let mut s = self.clone();
        for _ in 0..n {
            s = s.push(dir);
        }
        s
    }

    fn drop(&self) -> Shape {
        Shape(self.0.iter().map(|&(x, y)| (x, y - 1)).collect())
    }

    fn set_height(&mut self, height: isize) {
        self.0.iter_mut().for_each(|(_, y)| *y += height);
    }

    fn max_y(&self) -> isize {
        self.0.iter().map(|&(_, y)| y).max().unwrap()
    }
}

lazy_static! {
    static  ref SHAPES: Vec<Shape> = vec![
        Shape(vec![(0, 0), (1, 0), (2, 0), (3, 0)]), // horizontal bar
        Shape(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]), // cross
        Shape(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]), // J
        Shape(vec![(0, 0), (0, 1), (0, 2), (0, 3)]), // vertical bar
        Shape(vec![(0, 0), (1, 0), (0, 1), (1, 1)]), // square
    ]
    .iter()
    .map(|x| x.pushed(Dir::Right, 2))
    .collect();
}

#[derive(Debug)]
struct Chamber {
    width: isize,
    rocks: HashSet<Pos>,
    height: isize,
}

impl Chamber {
    fn new(width: isize) -> Self {
        Self {
            width,
            rocks: HashSet::new(),
            height: 0,
        }
    }

    fn overlaps(&self, shape: &Shape) -> bool {
        shape
            .0
            .iter()
            .any(|pos @ &(x, y)| x < 0 || x >= self.width || y < 0 || self.rocks.contains(pos))
    }

    fn add_shape(&mut self, shape: &Shape) {
        for &pos in &shape.0 {
            self.rocks.insert(pos);
        }
        self.height = max(self.height, shape.max_y() + 1);

        // prune
        if self.height % 10 == 0 {
            for p in self.rocks.clone() {
                if p.1 < self.height - 10 {
                    self.rocks.remove(&p);
                }
            }
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height).rev() {
            write!(f, "|")?;
            for x in 0..self.width {
                let c = if self.rocks.contains(&(x, y)) {
                    "#"
                } else {
                    "."
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "|")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "|")?;
        Ok(())
    }
}

fn play(instructions: &[Dir], n: usize) -> Chamber {
    let mut chamber = Chamber::new(7);
    let mut instructions_iter = instructions.iter().cycle();
    let mut cnt: isize = 0;
    for mut shape in SHAPES.iter().cycle().take(n).cloned() {
        cnt += 1;
        if cnt % 1_000_000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
        shape.set_height(chamber.height + 3);
        loop {
            let instruction = instructions_iter.next().unwrap();

            // try pushing
            let new_shape = shape.push(*instruction);
            if !chamber.overlaps(&new_shape) {
                shape = new_shape;
            }

            // try dropping
            let new_shape = shape.drop();
            if chamber.overlaps(&new_shape) {
                chamber.add_shape(&shape);
                break;
            }
            shape = new_shape;
        }
    }
    chamber
}

fn parse(input: &str) -> Vec<Dir> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None,
        })
        .collect()
}

pub fn solution1(input: &[String]) -> isize {
    play(&parse(input[0].as_str()), 2022).height
}

pub fn solution2(input: &[String]) -> isize {
    play(&parse(input[0].as_str()), 1_000_000_000_000).height
}

#[cfg(test)]
mod tests {
    use crate::day17::{parse, Dir};

    fn data() -> Vec<String> {
        vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".into()]
    }

    #[test]
    fn test_parse() {
        assert_eq!(vec![Dir::Left, Dir::Right, Dir::Left], parse("<><"));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(3068, super::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(1_514_285_714_288, super::solution2(&data()));
    }
}
