use pathfinding::prelude::astar;

type Pos = (usize, usize);

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
}

impl Grid {
    fn new(input: &[String]) -> Self {
        let mut grid: Vec<_> = input.iter().map(|v| v.bytes().collect()).collect();
        let width = input[0].len();
        let height = input.len();
        let start = find_all(&grid, b'S')[0];
        let end = find_all(&grid, b'E')[0];
        grid[start.1][start.0] = b'a';
        grid[end.1][end.0] = b'z';
        Self {
            grid,
            width,
            height,
            start,
            end,
        }
    }

    fn get(&self, pos: Pos) -> u8 {
        self.grid[pos.1][pos.0]
    }

    fn neighbours(&self, pos: Pos) -> Vec<Pos> {
        let mut result = vec![];
        if pos.0 > 0 {
            result.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.width - 1 {
            result.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            result.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.height - 1 {
            result.push((pos.0, pos.1 + 1));
        }
        result
    }

    fn successors(&self, pos: Pos) -> Vec<(Pos, usize)> {
        let cur = self.get(pos);
        self.neighbours(pos)
            .iter()
            .filter(|p| self.get(**p) <= cur + 1)
            .copied()
            .map(|x| (x, 1))
            .collect()
    }

    fn distance(pos1: Pos, pos2: Pos) -> usize {
        pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
    }

    fn dist_to_end(&self, pos: Pos) -> usize {
        Self::distance(pos, self.end)
    }
}

fn find_all(g: &[Vec<u8>], x: u8) -> Vec<Pos> {
    let mut result = vec![];
    for (y, row) in g.iter().enumerate() {
        if let Some(x) = row.iter().position(|c| *c == x) {
            result.push((x, y));
        }
    }
    result
}

pub fn solution1(data: &[String]) -> usize {
    let grid = Grid::new(data);
    let (_path, distance) = astar(
        &grid.start,
        |p| grid.successors(*p),
        |p| grid.dist_to_end(*p),
        |p| *p == grid.end,
    )
    .unwrap();
    distance
}

pub fn solution2(data: &[String]) -> usize {
    let grid = Grid::new(data);
    find_all(&grid.grid, b'a')
        .iter()
        .filter_map(|p| {
            astar(
                p,
                |p| grid.successors(*p),
                |p| grid.dist_to_end(*p),
                |p| *p == grid.end,
            )
        })
        .map(|x| x.1)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day12};

    fn data() -> Vec<String> {
        str2lines(
            r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(31, day12::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(29, day12::solution2(&data()));
    }
}
