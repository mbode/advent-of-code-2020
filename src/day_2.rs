use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<min>\d+)-(?P<max>\d+) (?P<c>.): (?P<password>.*)")]
struct PasswordPolicy {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid_1(&self) -> bool {
        let occurrences = self.password.matches(self.c).count();
        self.min <= occurrences && occurrences <= self.max
    }

    fn is_valid_2(&self) -> bool {
        (self.password.chars().nth(self.min - 1).unwrap() == self.c)
            ^ (self.password.chars().nth(self.max - 1).unwrap() == self.c)
    }
}

pub fn day_2_1(lines: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|line| line.parse::<PasswordPolicy>().ok())
        .filter(|p| p.is_valid_1())
        .count()
}

pub fn day_2_2(lines: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|line| line.parse::<PasswordPolicy>().ok())
        .filter(|p| p.is_valid_2())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day_2::{day_2_1, day_2_2};

    fn test_data() -> Vec<String> {
        vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ]
    }

    #[test]
    fn test_day_2_1() {
        assert_eq!(day_2_1(&test_data()), 2);
    }

    #[test]
    fn test_day_2_2() {
        assert_eq!(day_2_2(&test_data()), 1);
    }
}
