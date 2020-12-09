pub fn day_9_1(numbers: &[usize], span: usize) -> Option<usize> {
    'outer: for i in span..numbers.len() {
        let target = &numbers[i];
        for j in i - span..i {
            for k in i - span..i {
                if j != k && numbers[j] + numbers[k] == *target {
                    continue 'outer;
                }
            }
        }
        return Some(*target);
    }
    None
}

pub fn day_9_2(numbers: &[usize], target: usize) -> Option<usize> {
    for start in 0..numbers.len() {
        for end in start + 1..numbers.len() {
            let range = &numbers[start..end];
            let sum = range.iter().sum::<usize>();
            if sum > target {
                break;
            }
            if sum == target {
                return Some(range.iter().min().unwrap() + range.iter().max().unwrap());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::day_9::{day_9_1, day_9_2};

    fn test_data() -> Vec<usize> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn test_day_9_1() {
        assert_eq!(day_9_1(&test_data(), 5).unwrap(), 127);
    }

    #[test]
    fn test_day_9_2() {
        assert_eq!(day_9_2(&test_data(), 127).unwrap(), 62);
    }
}
