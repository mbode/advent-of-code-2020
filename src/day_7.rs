use std::collections::{HashMap, HashSet};

pub fn day_7_1(lines: &[String]) -> usize {
    let mut contained_in = HashMap::new();
    for line in lines {
        let outer = line.split(" bags contain").next().unwrap();
        let second_part = line.split("contain ").nth(1).unwrap();

        let inners = second_part.split(", ");
        for inner in inners {
            let words: Vec<String> = inner.split(' ').map(String::from).collect();
            if words[0] == "no" {
                continue;
            }
            let inner_bag = format!("{} {}", words[1], words[2]);
            match contained_in.get(&inner_bag) {
                None => {
                    let mut new_set = HashSet::new();
                    new_set.insert(outer);
                    contained_in.insert(inner_bag.clone(), new_set);
                }
                Some(existing_set) => {
                    let mut new_set = HashSet::new();
                    new_set.extend(existing_set);
                    new_set.insert(outer);
                    contained_in.insert(inner_bag.clone(), new_set);
                }
            }
        }
    }

    insert(&contained_in, "shiny gold").len()
}

fn insert<'a>(contained_in: &'a HashMap<String, HashSet<&str>>, color: &str) -> HashSet<&'a str> {
    let mut candidates = HashSet::new();
    for candidate in contained_in.get(color).unwrap() {
        candidates.insert(*candidate);
        if contained_in.contains_key(*candidate) {
            candidates.extend(insert(contained_in, candidate));
        }
    }
    candidates
}

pub fn day_7_2(lines: &[String]) -> usize {
    let mut contains = HashMap::new();
    for line in lines {
        let outer = line.split(" bags contain").next().unwrap();
        let second_part = line.split("contain ").nth(1).unwrap();

        let inners = second_part.split(", ");
        let mut inner_bags_with_numbers = HashSet::new();
        for inner in inners {
            let words: Vec<String> = inner.split(' ').map(String::from).collect();
            if words[0] == "no" {
                continue;
            }
            let number = words[0].parse::<usize>().unwrap();
            let inner_bag = format!("{} {}", words[1], words[2]);
            inner_bags_with_numbers.insert((inner_bag, number));
        }
        contains.insert(outer, inner_bags_with_numbers);
    }
    count_bags(&contains, "shiny gold")
}

fn count_bags(contains: &HashMap<&str, HashSet<(String, usize)>>, color: &str) -> usize {
    contains
        .get(color)
        .unwrap()
        .iter()
        .map(|(inner, number)| number * (1 + count_bags(contains, inner)))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_7::{day_7_1, day_7_2};

    fn test_data() -> Vec<String> {
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_7_1() {
        assert_eq!(day_7_1(&test_data()), 4);
    }

    #[test]
    fn test_day_7_2() {
        assert_eq!(day_7_2(&test_data()), 32);
    }
}
