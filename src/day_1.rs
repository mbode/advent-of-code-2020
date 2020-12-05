pub fn day_1_1(numbers: &[isize]) -> Option<isize> {
    for i in numbers {
        for j in numbers {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }
    None
}

pub fn day_1_2(numbers: &[isize]) -> Option<isize> {
    for i in numbers {
        for j in numbers {
            for k in numbers {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::day_1::{day_1_1, day_1_2};

    fn test_data() -> Vec<isize> {
        vec![1721, 979, 366, 299, 675, 1456]
    }

    #[test]
    fn test_day_1_1() {
        assert_eq!(day_1_1(&test_data()).unwrap(), 514579);
    }

    #[test]
    fn test_day_1_2() {
        assert_eq!(day_1_2(&test_data()).unwrap(), 241861950);
    }
}
