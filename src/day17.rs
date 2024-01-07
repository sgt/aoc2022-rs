use lazy_static::lazy_static;
use std::{
    fmt::Display,
    iter::{Cycle, Enumerate},
    vec::IntoIter,
};

const CHAMBER_WIDTH: u8 = 7;
const BUFFER_HEIGHT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Piece {
    data: Vec<u8>,
    y: usize, // y-position in chamber (by lowest row) (absolute)
}

impl Piece {
    pub fn new(data: &[u8], y: usize) -> Self {
        Self {
            data: data.to_vec(),
            y,
        }
    }

    pub fn is_leftmost(&self) -> bool {
        self.data.iter().any(|x| x & 1 << (CHAMBER_WIDTH - 1) != 0)
    }

    pub fn is_rightmost(&self) -> bool {
        self.data.iter().any(|x| x & 0b1 != 0)
    }

    fn push(&mut self, dir: Dir) {
        if (dir == Dir::Left && self.is_leftmost()) || (dir == Dir::Right && self.is_rightmost()) {
            return;
        }

        for row in &mut self.data {
            match dir {
                Dir::Left => *row <<= 1,
                Dir::Right => *row >>= 1,
            }
        }
    }

    fn dropped(&self) -> Self {
        let mut p = self.clone();
        p.y -= 1;
        p
    }

    fn pushed(&self, dir: Dir) -> Self {
        let mut p = self.clone();
        p.push(dir);
        p
    }

    fn pushed_n(&mut self, dir: Dir, n: usize) -> Self {
        let mut p = self.clone();
        for _ in 0..n {
            p.push(dir);
        }
        p
    }
}

lazy_static! {
    static ref SHAPES: Vec<Vec<u8>> =  vec![
        vec![
            0b0111_1000,
        ], // horizontal bar
        vec![
            0b0010_0000,
            0b0111_0000,
            0b0010_0000,
        ], // cross
        vec![
            0b0111_0000,
            0b0001_0000,
            0b0001_0000,
        ], // J
        vec![
            0b0100_0000,
            0b0100_0000,
            0b0100_0000,
            0b0100_0000,
        ], // vertical bar
        vec![
            0b0110_0000,
            0b0110_0000,
        ], // square
    ];
}

#[derive(Debug, Clone)]
struct Chamber {
    rocks: Vec<u8>,
}

impl Chamber {
    fn new() -> Self {
        Self { rocks: vec![] }
    }

    // absolute height
    fn height(&self) -> usize {
        self.rocks.len()
    }

    fn overlaps(&self, piece: &Piece) -> bool {
        piece.data.iter().enumerate().any(|(y, row)| {
            let abs_piece_y = y + piece.y;
            abs_piece_y < self.rocks.len() && row & self.rocks[abs_piece_y] != 0
        })
    }

    fn add_piece(&mut self, piece: &Piece) {
        let max_piece_height = piece.y + piece.data.len();
        if self.rocks.len() < max_piece_height {
            self.rocks.resize(max_piece_height, 0);
        }
        for (y, row) in piece.data.iter().enumerate() {
            self.rocks[y + piece.y] |= row;
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rocks.iter().rev() {
            write!(f, "|")?;
            for x in (0..CHAMBER_WIDTH).rev() {
                let c = if (1 << x) & *row == 0 { "." } else { "#" };
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "|")?;
        for _ in 0..CHAMBER_WIDTH {
            write!(f, "-")?;
        }
        writeln!(f, "|")?;
        Ok(())
    }
}

// fn determine_cycle()

struct Game {
    chamber: Chamber,
    instructions_iter: Cycle<Enumerate<IntoIter<Dir>>>,
    shapes_iter: Cycle<Enumerate<IntoIter<Vec<u8>>>>,
}

impl Game {
    pub fn new(instructions: &[Dir]) -> Game {
        Game {
            chamber: Chamber::new(),
            #[allow(clippy::unnecessary_to_owned)]
            instructions_iter: instructions.to_vec().into_iter().enumerate().cycle(),
            shapes_iter: SHAPES.clone().into_iter().enumerate().cycle(),
        }
    }

    fn play_next_piece(&mut self) {
        let (_, shape) = self.shapes_iter.next().unwrap();
        let mut piece = Piece::new(&shape, self.chamber.height() + 3).pushed_n(Dir::Right, 2);

        loop {
            let (_, instr) = self.instructions_iter.next().unwrap();

            // try pushing
            let new_piece = piece.pushed(instr);
            if !self.chamber.overlaps(&new_piece) {
                piece = new_piece;
            }

            if piece.y == 0 {
                // reached floor
                self.chamber.add_piece(&piece);
                break;
            }

            // try dropping
            let new_piece = piece.dropped();
            if self.chamber.overlaps(&new_piece) {
                self.chamber.add_piece(&piece);
                break;
            }
            piece = new_piece;
        }
    }

    fn play_n(&mut self, n: usize) {
        for _ in 0..n {
            self.play_next_piece();
        }
    }
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

pub fn solution1(input: &[String]) -> usize {
    let mut game = Game::new(&parse(input[0].as_str()));
    game.play_n(2022);
    game.chamber.height()
}

pub fn solution2(input: &[String]) -> usize {
    let mut game = Game::new(&parse(input[0].as_str()));
    game.play_n(1_000_000_000_000);
    game.chamber.height()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day17::{parse, Dir};

    fn data() -> Vec<String> {
        vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".into()]
    }

    #[test]
    fn test_parse() {
        assert_eq!(vec![Dir::Left, Dir::Right, Dir::Left], parse("<><"));
    }

    #[test]
    fn test_overlaps() {
        let chamber = Chamber::new();
        let piece = Piece::new(&[0b0111_1000], 0);
        assert!(!chamber.overlaps(&piece), "no overlap for empty chamber");

        let mut chamber2 = Chamber::new();
        chamber2.rocks = vec![0b0011_0000, 0b0011_0000];
        let mut piece2 = Piece::new(&[0b0011_1000, 0b0000_1000, 0b0000_10000], 0);
        assert!(chamber2.overlaps(&piece2));
        piece2.y += 1;
        assert!(chamber2.overlaps(&piece2));
        piece2.y += 1;
        assert!(!chamber2.overlaps(&piece2));
    }

    #[test]
    fn test_add_piece() {
        let mut chamber = Chamber::new();
        let piece = Piece::new(&[0b0111_1000], 0);
        chamber.add_piece(&piece);
        assert_eq!(chamber.rocks, vec![0b0111_1000]);

        let mut chamber2 = Chamber::new();
        chamber2.rocks = vec![0b0011_0000, 0b0011_0000];
        let piece2 = Piece::new(&[0b0111_0000, 0b0001_0000, 0b0001_0000], 2);
        chamber2.add_piece(&piece2);
        assert_eq!(
            chamber2.rocks,
            vec![
                0b0011_0000,
                0b0011_0000,
                0b0111_0000,
                0b0001_0000,
                0b0001_0000,
            ] // chamber is stored bottom-to-top, unlike the pieces
        );
    }

    #[test]
    fn test_play_n() {
        let instructions = parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let mut game = Game::new(&instructions);
        game.play_n(3);
        assert_eq!(game.chamber.height(), 6);

        let mut game2 = Game::new(&instructions);
        game2.play_n(10);
        assert_eq!(game2.chamber.height(), 17);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(3068, solution1(&data()));
    }

    #[test]
    #[ignore]
    fn test_solution2() {
        assert_eq!(1_514_285_714_288, solution2(&data()));
    }
}
