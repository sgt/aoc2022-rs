use std::collections::HashSet;

use crate::common::transpose;

fn parse(data: &[String]) -> Vec<Vec<u8>> {
    data.iter().map(|row| row.as_bytes().into()).collect()
}

/// Returns vector index after which it stops being monotonously increasing.
fn visible_from_left<T: PartialOrd>(arr: &[T]) -> HashSet<usize> {
    // panics if v is empty but we don't care
    let mut max = &arr[0];
    let mut result = HashSet::from([0]);
    for (i, el) in arr.iter().enumerate().skip(1) {
        if el > max {
            max = el;
            result.insert(i);
        }
    }
    result
}

/// Check visibility for both sides of a tree row.
fn visible_left_and_right<T: PartialOrd + Copy>(arr: &[T]) -> HashSet<usize> {
    let left = visible_from_left(arr);
    let reversed: Vec<_> = arr.iter().rev().collect();
    let right = visible_from_left(&reversed)
        .iter()
        .map(|x| arr.len() - x - 1)
        .collect();
    left.union(&right).copied().collect()
}

pub fn solution1(data: &[String]) -> usize {
    let input = parse(data);
    let mut result = HashSet::new();

    for (j, line) in input.iter().enumerate() {
        let coords = visible_left_and_right(line);
        for x in coords {
            result.insert((x, j));
        }
    }

    for (i, row) in transpose(&input).iter().enumerate() {
        let coords = visible_left_and_right(row);
        for x in coords {
            result.insert((i, x));
        }
    }

    result.len()
}

pub fn solution2(data: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{common::str2lines, day8};

    fn data() -> Vec<String> {
        str2lines(
            r#"30373
25512
65332
33549
35390"#,
        )
    }

    #[test]
    fn test_visible_boundaries() {
        assert_eq!(HashSet::from([0]), day8::visible_left_and_right(b"5"));
        assert_eq!(HashSet::from([0, 2]), day8::visible_left_and_right(b"515"));
        assert_eq!(
            HashSet::from([0, 1, 2, 4]),
            day8::visible_left_and_right(b"25512")
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(21, day8::solution1(&data()));
    }
}
