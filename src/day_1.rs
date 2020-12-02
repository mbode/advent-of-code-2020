pub fn day_1_1(numbers: &Vec<i32>) -> Option<i32> {
    for i in numbers {
        for j in numbers {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }
    None
}

pub fn day_1_2(numbers: &Vec<i32>) -> Option<i32> {
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

    #[test]
    fn test_day_1_1() {
        let entries = vec!(1721, 979, 366, 299, 675, 1456);
        assert_eq!(day_1_1(&entries).unwrap(), 514579);
    }

    fn test_day_1_2() {
        let entries = vec!(1721, 979, 366, 299, 675, 1456);
        assert_eq!(day_1_2(&entries).unwrap(), 514579);
    }
}