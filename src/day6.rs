fn find_n_unique(n: usize, s: &str) -> Option<usize> {
    let mut sofar: String = "".into();

    for (i, c) in s.chars().enumerate() {
        if let Some(x) = sofar.find(c) {
            sofar.drain(..x + 1);
        }
        sofar.push(c);
        if sofar.len() == n {
            return Some(i + 1);
        }
    }

    None
}

pub fn solution1(data: &[String]) -> usize {
    find_n_unique(4, &data[0]).unwrap()
}

pub fn solution2(data: &[String]) -> usize {
    find_n_unique(14, &data[0]).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day6;

    #[test]
    fn test_solution1() {
        assert_eq!(
            7,
            day6::solution1(&["mjqjpqmgbljsphdztnvjfqwrcgsmlb".into()])
        );
        assert_eq!(5, day6::solution1(&["bvwbjplbgvbhsrlpgdmjqwftvncz".into()]));
        assert_eq!(6, day6::solution1(&["nppdvjthqldpwncqszvftbrmjlhg".into()]));
        assert_eq!(
            10,
            day6::solution1(&["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into()])
        );
        assert_eq!(
            11,
            day6::solution1(&["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into()])
        );
    }
}
