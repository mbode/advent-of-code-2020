use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day_6_1(lines: &[String]) -> usize {
    lines
        .split(|l| l == "")
        .map(|group| {
            let mut answered = HashSet::new();
            for line in group {
                for c in line.chars() {
                    answered.insert(c);
                }
            }
            answered.len()
        })
        .sum()
}

pub fn day_6_2(lines: &[String]) -> usize {
    lines
        .split(|l| l == "")
        .map(|group| {
            let mut answered: HashSet<char> =
                HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
            for line in group {
                for c in answered.clone() {
                    if !line.contains(c) {
                        answered.remove(&c);
                    }
                }
            }
            answered.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_6::{day_6_1, day_6_2};

    fn test_data() -> Vec<String> {
        "abc

a
b
c

ab
ac

a
a
a
a

b"
        .lines()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_day_6_1() {
        assert_eq!(day_6_1(&test_data()), 11);
    }

    #[test]
    fn test_day_6_2() {
        assert_eq!(day_6_2(&test_data()), 6);
    }
}
