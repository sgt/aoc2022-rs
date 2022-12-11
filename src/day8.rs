use std::{cmp::max, collections::HashSet};

use crate::common::transpose;

fn parse(data: &[String]) -> Vec<Vec<u8>> {
    data.iter().map(|row| row.as_bytes().into()).collect()
}

/// Returns vector index after which it stops being monotonously increasing.
fn visible_l<T: PartialOrd>(arr: &[T]) -> HashSet<usize> {
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
fn visible_lr<T: PartialOrd>(arr: &[T]) -> HashSet<usize> {
    let left = visible_l(arr);
    let reversed: Vec<_> = arr.iter().rev().collect();
    let right = visible_l(&reversed)
        .iter()
        .map(|x| arr.len() - x - 1)
        .collect();
    left.union(&right).copied().collect()
}

fn scenic_score_l<T: PartialOrd>(arr: &[T], idx: usize) -> usize {
    let val = &arr[idx];
    let mut count = 0;
    for i in (0..idx).rev() {
        count += 1;
        if &arr[i] >= val {
            break;
        }
    }
    count
}

fn scenic_score_lr<T: PartialOrd>(arr: &[T], idx: usize) -> usize {
    let l = scenic_score_l(arr, idx);
    let x: Vec<_> = arr.iter().rev().collect();
    let r = scenic_score_l(&x, arr.len() - idx - 1);
    l * r
}

fn scenic_score_line_lr<T: PartialOrd>(line: &[T]) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..line.len() {
        result.push(scenic_score_lr(line, i));
    }
    result
}

fn scenic_score_matrix_lr<T: PartialOrd>(arr: &[Vec<T>]) -> Vec<Vec<usize>> {
    arr.iter().map(|line| scenic_score_line_lr(line)).collect()
}

pub fn solution1(data: &[String]) -> usize {
    let input = parse(data);
    let mut result = HashSet::new();

    for (j, line) in input.iter().enumerate() {
        let coords = visible_lr(line);
        for x in coords {
            result.insert((x, j));
        }
    }

    for (i, row) in transpose(&input).iter().enumerate() {
        let coords = visible_lr(row);
        for x in coords {
            result.insert((i, x));
        }
    }

    result.len()
}

pub fn solution2(data: &[String]) -> usize {
    let input = parse(data);
    let horiz: Vec<usize> = scenic_score_matrix_lr(&input).into_iter().flatten().collect();
    let transposed = transpose(&input);
    let transposed_vert = scenic_score_matrix_lr(&transposed);
    // a bit of a mess since I decided to make transpose return &[&T]
    let vert: Vec<usize> = transpose(&transposed_vert)
        .iter()
        .flatten()
        .copied()
        .copied()
        .collect();
    let mut result = 0;
    for (i, x) in horiz.into_iter().enumerate() {
        result = max(result, x * vert[i]);
    }
    result
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
    fn test_visible_lr() {
        assert_eq!(HashSet::from([0]), day8::visible_lr(b"5"));
        assert_eq!(HashSet::from([0, 2]), day8::visible_lr(b"515"));
        assert_eq!(HashSet::from([0, 1, 2, 4]), day8::visible_lr(b"25512"));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn test_scenic_score_left_and_right() {
        assert_eq!(0, day8::scenic_score_lr(b"33549", 0));
        assert_eq!(2 * 2, day8::scenic_score_lr(b"33549", 2));
        assert_eq!(2 * 1, day8::scenic_score_lr(b"35353", 3));
        assert_eq!(1 * 1, day8::scenic_score_lr(b"95653", 3));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(21, day8::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(8, day8::solution2(&data()));
    }
}
