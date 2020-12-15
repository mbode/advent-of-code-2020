use std::collections::HashMap;

pub fn day_15_1(numbers: &[usize]) -> usize {
    day_15(numbers, 2020)
}

pub fn day_15_2(numbers: &[usize]) -> usize {
    day_15(numbers, 30000000)
}

fn day_15(numbers: &[usize], target: usize) -> usize {
    let mut v: Vec<usize> = numbers.to_vec();
    let mut last_seen: HashMap<usize, usize> =
        v.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect();

    for i in v.len() - 1..target - 1 {
        let last = v[i];
        match last_seen.insert(last, i) {
            None => {
                v.push(0);
            }
            Some(j) => {
                v.push(i - j);
            }
        }
    }
    v[target - 1]
}

#[cfg(test)]
mod tests {
    use crate::day_15::{day_15_1, day_15_2};

    #[test]
    fn test_day_15_1() {
        assert_eq!(day_15_1(&vec![0, 3, 6]), 436);
        assert_eq!(day_15_1(&vec![1, 3, 2]), 1);
        assert_eq!(day_15_1(&vec![2, 1, 3]), 10);
        assert_eq!(day_15_1(&vec![1, 2, 3]), 27);
        assert_eq!(day_15_1(&vec![2, 3, 1]), 78);
        assert_eq!(day_15_1(&vec![3, 2, 1]), 438);
        assert_eq!(day_15_1(&vec![3, 1, 2]), 1836);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_a() {
        assert_eq!(day_15_2(&vec![0, 3, 6]), 175594);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_b() {
        assert_eq!(day_15_2(&vec![1, 3, 2]), 2578);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_c() {
        assert_eq!(day_15_2(&vec![2, 1, 3]), 3544142);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_d() {
        assert_eq!(day_15_2(&vec![1, 2, 3]), 261214);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_e() {
        assert_eq!(day_15_2(&vec![2, 3, 1]), 6895259);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_f() {
        assert_eq!(day_15_2(&vec![3, 2, 1]), 18);
    }

    #[test]
    #[ignore]
    fn test_day_15_2_g() {
        assert_eq!(day_15_2(&vec![3, 1, 2]), 362);
    }
}
