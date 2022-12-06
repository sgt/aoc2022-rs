fn vec_of_nums(v: &[String]) -> Vec<i32> {
    v.iter()
        .map(|x| x.parse().expect("invalid integer"))
        .collect()
}

pub(crate) fn int_groups_data(data: &[String]) -> Vec<Vec<i32>> {
    data.split(|v| v.is_empty()).map(vec_of_nums).collect()
}

#[cfg(test)]
pub(crate) fn str2lines(s: &str) -> Vec<String> {
    s.lines().map(|x| x.into()).collect()
}
