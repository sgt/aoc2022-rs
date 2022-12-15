use std::cmp::Ordering;

use json::JsonValue;

type DataPair = (JsonValue, JsonValue);

fn parse(data: &[String]) -> Vec<DataPair> {
    data.split(|x| x.is_empty())
        .map(|v| {
            v.iter()
                .map(|x| json::parse(x).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|a| (a[0].clone(), a[1].clone()))
        .collect()
}

fn compare(v1: &JsonValue, v2: &JsonValue) -> Ordering {
    if let (Some(i1), Some(i2)) = (v1.as_i32(), v2.as_i32()) {
        i1.cmp(&i2)
    } else if v1.is_array() && v2.is_array() {
        if v1.is_empty() && v2.is_empty() {
            Ordering::Equal
        } else if v1.is_empty() && !v2.is_empty() {
            Ordering::Less
        } else if !v1.is_empty() && v2.is_empty() {
            Ordering::Greater
        } else {
            match compare(&v1[0], &v2[0]) {
                Ordering::Equal => {
                    let (mut a1, mut a2) = (v1.clone(), v2.clone());
                    match compare(&a1.array_remove(0), &a2.array_remove(0)) {
                        Ordering::Equal => compare(&a1,&a2),
                        x => x,
                    }
                }
                x => x,
            }
        }
    } else if v1.is_number() && v2.is_array() {
        compare(&JsonValue::Array(vec![v1.clone()]), v2)
    } else if v1.is_array() && v2.is_number() {
        compare(v1, &JsonValue::Array(vec![v2.clone()]))
    } else {
        panic!("unexpected case: {} {}", v1, v2)
    }
}

pub fn solution1(input: &[String]) -> usize {
    parse(input)
        .iter()
        .map(|(a, b)| compare(a, b))
        .enumerate()
        .filter(|(_, x)| *x == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solution2(_input: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day13};

    fn data() -> Vec<String> {
        str2lines(
            r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(13, day13::solution1(&data()));
    }
}
