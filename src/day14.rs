use std::{
    cmp::{max, min},
    collections::HashSet,
};

type Pos = (usize, usize);

struct Line(Pos, Pos);

impl Line {
    fn positions(&self) -> Vec<Pos> {
        let mut result = vec![];
        for x in min(self.0 .0, self.1 .0)..=max(self.0 .0, self.1 .0) {
            for y in min(self.0 .1, self.1 .1)..=max(self.0 .1, self.1 .1) {
                result.push((x, y));
            }
        }
        result
    }

    fn str2pos(s: &str) -> Pos {
        let (s1, s2) = s.split_once(',').unwrap();
        (s1.parse().unwrap(), s2.parse().unwrap())
    }
}

struct Grid {
    rock: HashSet<Pos>,
    sand: HashSet<Pos>,
    max_depth: usize,
}

impl Grid {
    fn new(data: &[Line]) -> Self {
        let mut rock = HashSet::new();
        for line in data {
            rock.extend(line.positions());
        }
        let max_depth = Self::max_depth(&rock);
        Self {
            rock,
            sand: HashSet::new(),
            max_depth,
        }
    }

    fn is_taken(&self, pos: &Pos) -> bool {
        self.rock.contains(pos) || self.sand.contains(pos)
    }

    /// Calculates where sand will land without actually adding it to the grid. None = will fall off the grid.
    fn sand_land_position(&self, x: usize) -> Option<Pos> {
        let mut cur_pos = (x, 0);
        'falling: while cur_pos.1 < self.max_depth {
            for next_pos in [
                (cur_pos.0, cur_pos.1 + 1),
                (cur_pos.0 - 1, cur_pos.1 + 1),
                (cur_pos.0 + 1, cur_pos.1 + 1),
            ] {
                if !self.is_taken(&next_pos) {
                    cur_pos = next_pos;
                    continue 'falling;
                }
            }
            return Some(cur_pos);
        }
        None
    }

    fn max_depth(rock: &HashSet<Pos>) -> usize {
        rock.iter().map(|x| x.1).max().unwrap()
    }
}

fn parse(input: &[String]) -> Vec<Line> {
    input
        .iter()
        .flat_map(|x| {
            x.split(" -> ")
                .map(Line::str2pos)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|a| Line(a[0], a[1]))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn solution1(input: &[String]) -> usize {
    let mut grid = Grid::new(&parse(input));
    while let Some(pos) = grid.sand_land_position(500) {
        grid.sand.insert(pos);
    }
    grid.sand.len()
}

pub fn solution2(_input: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day14};

    fn data() -> Vec<String> {
        str2lines(
            r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(24, day14::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(0, day14::solution2(&data()));
    }
}
