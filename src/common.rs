pub(crate) fn vec_of_nums(v: &[String]) -> Vec<i32> {
    v.iter()
        .map(|x| x.parse().expect("invalid integer"))
        .collect()
}

pub(crate) fn int_groups_data(data: &[String]) -> Vec<Vec<i32>> {
    data.split(String::is_empty).map(vec_of_nums).collect()
}

#[cfg(test)]
pub(crate) fn str2lines(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}

pub(crate) fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    // panics if the vec is not square but who cares
    if let Some(l1) = v.first() {
        let mut result = vec![];
        for i in 0..l1.len() {
            let mut line = vec![];
            for l in v {
                line.push(&l[i]);
            }
            result.push(line);
        }
        result
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::common::transpose;

    #[test]
    fn test_transpose() {
        let x = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let xt = vec![vec![&1, &4], vec![&2, &5], vec![&3, &6]];
        assert_eq!(xt, transpose(&x));
    }
}
