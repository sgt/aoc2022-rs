use crate::common::int_groups_data;

fn elf_totals(data: &[String]) -> Vec<i32> {
    let groups = int_groups_data(data);
    groups.iter().map(|v| v.iter().sum()).collect()
}

pub fn solution1(data: &[String]) -> i32 {
    *elf_totals(data).iter().max().unwrap()
}

pub fn solution2(data: &[String]) -> i32 {
    let mut totals = elf_totals(data);
    totals.sort_unstable();
    totals.reverse();
    totals.iter().take(3).sum()
}

#[cfg(test)]
mod tests {

    use crate::{common::str2lines, day1};

    fn data() -> Vec<String> {
        str2lines(
            r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(24000, day1::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(45000, day1::solution2(&data()));
    }
}
