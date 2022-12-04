use std::collections::HashSet;

struct Backpack {
    c1: HashSet<u8>,
    c2: HashSet<u8>,
}

impl Backpack {
    fn str2items(s: &str) -> HashSet<u8> {
        HashSet::from_iter(s.bytes())
    }

    fn parse(s: &str) -> Self {
        let (c1, c2) = s.split_at(s.len() / 2);
        Backpack {
            c1: Self::str2items(c1),
            c2: Self::str2items(c2),
        }
    }

    fn priority(item: u8) -> u8 {
        if (b'a'..=b'z').contains(&item) {
            item - b'a' + 1
        } else {
            item - b'A' + 27
        }
    }

    fn common_items(&self) -> HashSet<u8> {
        self.c1.intersection(&self.c2).copied().collect()
    }

    fn total_common_priorities(&self) -> i32 {
        self.common_items()
            .iter()
            .map(|x| Self::priority(*x) as i32)
            .sum()
    }
}

pub fn solution1(data: &[String]) -> i32 {
    data.iter()
        .map(|x| Backpack::parse(x))
        .map(|x| x.total_common_priorities())
        .sum()
}

pub fn solution2(data: &[String]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day3};

    fn data() -> Vec<String> {
        str2lines(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(157, day3::solution1(&data()));
    }
    
    #[test]
    fn test_solution2() {
        assert_eq!(70, day3::solution2(&data()));
    }
}
