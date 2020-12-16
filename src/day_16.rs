use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

lazy_static! {
    static ref RANGES_REGEX: Regex = Regex::new(r"^ (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

pub fn day_16_1(lines: &[String]) -> usize {
    let mut section = 0;
    let mut sum = 0;

    let mut fields = HashMap::new();

    for line in lines {
        if line == "" {
            section += 1;
            continue;
        }
        match section {
            0 => {
                let mut split = line.split(':');
                let field_name = split.next().unwrap();
                let captures = RANGES_REGEX.captures(split.next().unwrap()).unwrap();
                fields.insert(
                    field_name,
                    (
                        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    ),
                );
            }
            1 => {}
            2 => {
                if line == "nearby tickets:" {
                    continue;
                }
                for i in line.split(',').map(|i| i.parse::<usize>().unwrap()) {
                    let mut found = false;
                    for value in fields.values() {
                        if (value.0 <= i && i <= value.1) || (value.2 <= i && i <= value.3) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        sum += i;
                    }
                }
            }
            _ => (),
        }
    }
    sum
}

pub fn day_16_2(lines: &[String]) -> usize {
    let mut section = 0;

    let num_fields = lines.iter().last().unwrap().split(',').count();

    let mut field_rules = HashMap::new();
    let mut field_candidates: HashMap<&str, HashSet<usize>> = HashMap::new();
    let mut your_ticket: Vec<usize> = Vec::new();
    for line in lines {
        if line == "" {
            section += 1;
            continue;
        }
        match section {
            0 => {
                let mut split = line.split(':');
                let field_name = split.next().unwrap();
                let captures = RANGES_REGEX.captures(split.next().unwrap()).unwrap();
                field_rules.insert(
                    field_name,
                    (
                        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    ),
                );
                field_candidates.insert(field_name, HashSet::from_iter(0..num_fields));
            }
            1 => {
                if line == "your ticket:" {
                    continue;
                }
                your_ticket = line
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect();
            }
            2 => {
                if line == "nearby tickets:" {
                    continue;
                }
                for (pos, i) in line
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .enumerate()
                {
                    let mut found = false;
                    let mut field_names = HashSet::new();
                    for (field_name, value) in &field_rules {
                        if (value.0 <= i && i <= value.1) || (value.2 <= i && i <= value.3) {
                            found = true;
                            field_names.insert(*field_name);
                        }
                    }
                    if !found {
                        continue;
                    }

                    for (name, set) in &field_candidates.clone() {
                        if !field_names.contains(name) {
                            let mut new_set = set.clone();
                            new_set.remove(&pos);
                            field_candidates.insert(name, new_set);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    let mut field_mapping = HashMap::new();
    while !field_candidates.is_empty() {
        for (name, set) in field_candidates.clone() {
            if set.len() == 1 {
                let i = *set.iter().next().unwrap();
                field_mapping.insert(name, i);
                field_candidates.remove(name);
                for (name, set) in field_candidates.clone() {
                    let mut new_set = set.clone();
                    new_set.remove(&i);
                    field_candidates.insert(name, new_set);
                }
            }
        }
    }

    field_mapping
        .iter()
        .filter(|(&name, _)| name.starts_with("departure"))
        .map(|(_, &pos)| your_ticket[pos])
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day_16::{day_16_1, day_16_2};

    fn test_data() -> Vec<String> {
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .lines()
            .map(String::from)
            .collect()
    }

    fn test_data_2() -> Vec<String> {
        "class: 0-1 or 4-19
departure_row: 0-5 or 8-19
departure_seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_16_1() {
        assert_eq!(day_16_1(&test_data()), 71);
    }

    #[test]
    fn test_day_16_2() {
        assert_eq!(day_16_2(&test_data_2()), 13 * 11);
    }
}
