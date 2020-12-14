use std::collections::{HashMap, HashSet};

pub fn day_14_1(lines: &[String]) -> usize {
    let mut mask_str = "";
    let mut memory = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask_str = line.split(" = ").nth(1).unwrap();
        } else {
            let mut iter = line.split("] = ");

            let adr = iter
                .next()
                .unwrap()
                .split('[')
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let n = iter.next().unwrap().parse::<usize>().unwrap();
            let n_str = format!("{:036b}", n);

            let mut result = 0;
            for i in 0..n_str.len() {
                match mask_str.chars().nth(i) {
                    Some('1') => {
                        result += 2_usize.pow(35 - i as u32);
                    }
                    Some('0') => (),
                    _ => {
                        if let Some('1') = n_str.chars().nth(i) {
                            result += 2_usize.pow(35 - i as u32);
                        }
                    }
                }
            }

            memory.insert(adr, result);
        }
    }
    memory.values().sum()
}

pub fn day_14_2(lines: &[String]) -> usize {
    let mut mask_str = "";
    let mut memory = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask_str = line.split(" = ").nth(1).unwrap();
        } else {
            let mut iter = line.split("] = ");

            let adr = iter
                .next()
                .unwrap()
                .split('[')
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let adr_str = format!("{:036b}", adr);

            let n = iter.next().unwrap().parse::<usize>().unwrap();

            let mut adrs = HashSet::new();
            adrs.insert(0);
            for i in 0..adr_str.len() {
                let previous_adrs = adrs.clone();
                adrs = HashSet::new();

                match mask_str.chars().nth(i) {
                    Some('X') => {
                        for adr in previous_adrs {
                            adrs.insert(adr);
                            adrs.insert(adr + 2_usize.pow(35 - i as u32));
                        }
                    }
                    Some('1') => {
                        for adr in previous_adrs {
                            adrs.insert(adr + 2_usize.pow(35 - i as u32));
                        }
                    }
                    Some('0') => match adr_str.chars().nth(i) {
                        Some('1') => {
                            for adr in previous_adrs {
                                adrs.insert(adr + 2_usize.pow(35 - i as u32));
                            }
                        }
                        Some('0') => {
                            for adr in previous_adrs {
                                adrs.insert(adr);
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            for adr in adrs {
                memory.insert(adr, n);
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_14::{day_14_1, day_14_2};

    fn test_data() -> Vec<String> {
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_14_1() {
        assert_eq!(day_14_1(&test_data()), 165);
    }

    fn test_data_2() -> Vec<String> {
        "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_14_2() {
        assert_eq!(day_14_2(&test_data_2()), 208);
    }
}
