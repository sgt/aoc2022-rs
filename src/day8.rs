use std::collections::HashSet;

use crate::common::transpose;

fn parse(data: &[String]) -> Vec<Vec<u8>> {
    data.iter().map(|row| row.as_bytes().into()).collect()
}

/// Returns vector index after which it stops being monotonously increasing.
fn mono_inc_until<T: PartialOrd>(arr: &[T]) -> usize {
    // panics if v is empty but we don't care
    let mut idx = 0;
    let mut prev = &arr[0];
    for i in &arr[1..] {
        if i <= prev {
            return idx;
        } else {
            prev = i;
            idx += 1;
        }
    }
    idx
}

/// Check visibility for both sides of a tree row.
fn visible_boundaries<T: PartialOrd>(arr: &[T]) -> (usize, usize) {
    let reversed: Vec<&T> = arr.iter().rev().collect();
    (
        mono_inc_until(arr),
        arr.len() - mono_inc_until(&reversed) - 1,
    )
}

pub fn solution1(data: &[String]) -> usize {
    let input = parse(data);
    let mut visible_trees = HashSet::new();

    for (j, line) in input.iter().enumerate() {
        let (i1, i2) = visible_boundaries(line);
        visible_trees.insert((i1, j));
        visible_trees.insert((i2, j));
    }

    for (i, row) in transpose(&input).iter().enumerate() {
        let (j1, j2) = visible_boundaries(row);
        visible_trees.insert((i, j1));
        visible_trees.insert((i, j2));
    }

    visible_trees.len()
}

pub fn solution2(data: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
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
        assert_eq!((0, 0), day8::visible_boundaries(&b"5".to_vec()));
        assert_eq!((0, 2), day8::visible_boundaries(&b"515".to_vec()));
        assert_eq!((1, 3), day8::visible_boundaries(&b"35390".to_vec()));
        assert_eq!((0, 4), day8::visible_boundaries(&b"33549".to_vec()));
        assert_eq!((0, 3), day8::visible_boundaries(&b"65332".to_vec()));
    }

    #[test]
    fn test_solution1() {}
}
