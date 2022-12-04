#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl MatchResult {
    fn score(&self) -> i32 {
        match self {
            MatchResult::Lose => 0,
            MatchResult::Draw => 3,
            MatchResult::Win => 6,
        }
    }
}

struct Instruction {
    opp: Shape,
    you: Shape,
}

impl Instruction {
    fn parse_letters(s: &str) -> (char, char) {
        let bs = s.as_bytes();
        (bs[0] as char, bs[2] as char)
    }

    fn parse1(s: &str) -> Self {
        let (opp, you) = Self::parse_letters(s);
        Instruction {
            opp: Self::parse_opp_letter(&opp),
            you: Self::parse_you_letter1(&you),
        }
    }

    fn parse2(s: &str) -> Self {
        let (opp, you) = Self::parse_letters(s);
        let opp_shape = Self::parse_opp_letter(&opp);
        Instruction {
            opp: opp_shape,
            you: Self::parse_you_letter2(&you, &opp_shape),
        }
    }

    fn parse_opp_letter(c: &char) -> Shape {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("unknown letter {}", c),
        }
    }

    fn parse_you_letter1(c: &char) -> Shape {
        match c {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("unknown letter {}", c),
        }
    }

    fn parse_you_letter2(c: &char, opp: &Shape) -> Shape {
        match c {
            'Y' => *opp,
            'X' => match opp {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            'Z' => match opp {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            _ => panic!("unknown letter {}", c),
        }
    }

    fn result(&self) -> MatchResult {
        match self {
            _ if self.opp == self.you => MatchResult::Draw,
            Instruction {
                opp: Shape::Paper,
                you: Shape::Scissors,
            }
            | Instruction {
                opp: Shape::Rock,
                you: Shape::Paper,
            }
            | Instruction {
                opp: Shape::Scissors,
                you: Shape::Rock,
            } => MatchResult::Win,
            _ => MatchResult::Lose,
        }
    }

    fn score(&self) -> i32 {
        self.result().score() + self.you.score()
    }
}

pub fn solution1(data: &[String]) -> i32 {
    data.iter()
        .map(|s| Instruction::parse1(s))
        .map(|x| x.score())
        .sum()
}

pub fn solution2(data: &[String]) -> i32 {
    data.iter()
        .map(|s| Instruction::parse2(s))
        .map(|x| x.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day2};

    fn data() -> Vec<String> {
        str2lines(
            r#"A Y
       B X
       C Z"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(15, day2::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(12, day2::solution2(&data()));
    }
}
