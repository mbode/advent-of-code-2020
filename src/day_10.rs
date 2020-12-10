use std::collections::HashMap;

pub fn day_10_1(lines: &[isize]) -> isize {
    let mut numbers: Vec<&isize> = lines.iter().collect();
    numbers.sort();
    let mut one = 0;
    let mut three = 0;
    let mut previous = 0;
    for n in numbers {
        match n - previous {
            1 => one += 1,
            3 => three += 1,
            _ => {}
        }
        previous = *n;
    }
    three += 1;
    one * three
}

pub fn day_10_2(lines: &[isize]) -> isize {
    let mut numbers: Vec<&isize> = lines.iter().collect();
    numbers.sort();

    let mut possible_ways = HashMap::new();
    possible_ways.insert(0, 1);
    for n in &numbers {
        let sum = vec![*n - 3, *n - 2, *n - 1]
            .iter()
            .map(|last| match possible_ways.get(&last) {
                None => 0,
                Some(i) => *i,
            })
            .sum();

        possible_ways.insert(**n, sum);
    }

    *possible_ways.get(numbers.last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_10::{day_10_1, day_10_2};

    fn test_data() -> Vec<isize> {
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
    }

    #[test]
    fn test_day_10_1() {
        assert_eq!(day_10_1(&test_data()), 220);
    }

    #[test]
    fn test_day_10_2() {
        assert_eq!(day_10_2(&test_data()), 19208);
    }
}
