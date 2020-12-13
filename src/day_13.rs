use num::integer::lcm;
use std::usize::MAX;

pub fn day_13_1(lines: &[String]) -> usize {
    let t: usize = lines[0].parse().unwrap();

    let busses: Vec<usize> = (&lines[1])
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut min_wait = MAX;
    let mut best_bus = 0;
    for bus in busses {
        let wait_time = ((t / bus) + 1) * bus - t;
        if wait_time < min_wait {
            min_wait = wait_time;
            best_bus = bus;
        }
    }
    min_wait * best_bus
}

pub fn day_13_2(lines: &[String]) -> usize {
    let busses: Vec<(usize, usize)> = (&lines[1])
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse::<usize>().unwrap()))
        .collect();

    let mut t = 0;
    let mut step = 1;
    let mut found = 0;
    'outer: loop {
        for (d, bus) in busses.iter().skip(found) {
            if (t + d) % bus == 0 {
                step = lcm(step, *bus);
                found += 1;
            } else {
                t += step;
                continue 'outer;
            }
        }
        return t;
    }
}

#[cfg(test)]
mod tests {
    use crate::day_13::{day_13_1, day_13_2};

    fn test_data() -> Vec<String> {
        "939
7,13,x,x,59,x,31,19"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_13_1() {
        assert_eq!(day_13_1(&test_data()), 295);
    }

    #[test]
    fn test_day_13_2() {
        assert_eq!(
            day_13_2(&vec![String::from(""), String::from("17,x,13,19")]),
            3417
        );
        assert_eq!(
            day_13_2(&vec![String::from(""), String::from("67,7,59,61")]),
            754018
        );
        assert_eq!(
            day_13_2(&vec![String::from(""), String::from("67,x,7,59,61")]),
            779210
        );
        assert_eq!(
            day_13_2(&vec![String::from(""), String::from("67,7,x,59,61")]),
            1261476
        );
        assert_eq!(
            day_13_2(&vec![String::from(""), String::from("1789,37,47,1889")]),
            1202161486
        );
    }
}
