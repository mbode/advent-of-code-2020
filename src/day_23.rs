pub fn day_23_1(input: &str) -> String {
    let mut circle: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for _ in 0..100 {
        let mut iter = circle.iter();
        let current = *iter.next().unwrap();

        let pick_up: Vec<usize> = iter.take(3).copied().collect();
        let mut destination = if current > 1 {
            current - 1
        } else {
            circle.len()
        };

        while pick_up.contains(&destination) {
            if destination > 1 {
                destination -= 1;
            } else {
                destination = circle.len();
            }
        }
        let mut i = 4;

        let mut new_circle = Vec::new();
        loop {
            new_circle.push(circle[i]);
            if circle[i] == destination {
                new_circle.extend(pick_up.iter())
            }
            if circle[i] == current {
                break;
            }
            i += 1;
            if i == circle.len() {
                i = 0;
            }
        }
        circle = new_circle;
    }

    let mut pos = circle.iter().position(|&e| e == 1).unwrap() + 1;
    let mut result = Vec::new();
    for _ in 0..circle.len() - 1 {
        result.push(circle[pos]);
        pos += 1;
        if pos == circle.len() {
            pos = 0;
        }
    }

    result.iter().map(|n| n.to_string()).collect()
}

pub fn day_23_2(input: &str) -> u64 {
    let mut circle: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    for i in *circle.iter().max().unwrap() + 1..1_000_000 + 1 {
        circle.push(i);
    }

    let mut linked = vec![0; circle.len() + 1];
    for window in circle.windows(2) {
        linked[window[0]] = window[1];
    }
    linked[*circle.iter().last().unwrap()] = circle[0];

    let mut current = circle[0];
    for _ in 0..10_000_000 {
        let mut destination = minus_one(current, linked.len());
        let first = linked[current];
        let second = linked[first];
        let third = linked[second];
        while vec![first, second, third].contains(&destination) {
            destination = minus_one(destination, linked.len())
        }
        let next = linked[third];
        linked[current] = next;
        linked[third] = linked[destination];
        linked[second] = third;
        linked[first] = second;
        linked[destination] = first;
        current = next;
    }

    let first = linked[1];
    first as u64 * linked[first] as u64
}

fn minus_one(current: usize, len: usize) -> usize {
    if current > 1 {
        current - 1
    } else {
        len - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::day_23::{day_23_1, day_23_2};

    #[test]
    fn test_day_23_1() {
        assert_eq!(day_23_1("389125467"), "67384529");
    }

    #[test]
    fn test_day_23_2() {
        assert_eq!(day_23_2("389125467"), 149245887792);
    }
}
