
pub fn day_n_1(lines: &[String]) -> usize  {
    1
}

pub fn day_n_2(lines: &[String]) -> usize  {
    2
}

#[cfg(test)]
mod tests {
    use crate::day_n::{day_n_1, day_n_2};

    fn test_data() -> Vec<String> {
        "a

a
b
c"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_n_1() {
        assert_eq!(day_n_1(&test_data()), 1);
    }

    #[test]
    fn test_day_n_2() {
        assert_eq!(day_n_2(&test_data()), 2);
    }
}