use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn day_21_1(lines: &[String]) -> usize {
    let (mut candidates, mut ingredient_count) = parse(lines);

    while !candidates.is_empty() {
        let (&a, set) = &candidates.iter().find(|(_, set)| set.len() == 1).unwrap();
        let &i = set.iter().next().unwrap();
        candidates.clone().iter().for_each(|(allergen, set)| {
            candidates.insert(allergen, set.iter().filter(|&s| *s != i).copied().collect());
        });
        candidates.remove(a);
        ingredient_count.remove(i);
    }

    ingredient_count.values().sum()
}

pub fn day_21_2(lines: &[String]) -> String {
    let (mut candidates, _) = parse(lines);

    let mut matched = HashMap::new();

    while !candidates.is_empty() {
        let (&a, set) = &candidates.iter().find(|(_, set)| set.len() == 1).unwrap();
        let &i = set.iter().next().unwrap();
        candidates.clone().iter().for_each(|(allergen, set)| {
            candidates.insert(allergen, set.iter().filter(|&s| *s != i).copied().collect());
        });
        candidates.remove(a);
        matched.insert(a, i);
    }

    matched
        .iter()
        .sorted_by_key(|(&a, _)| a)
        .map(|(_, &i)| i)
        .join(",")
}

fn parse(lines: &[String]) -> (HashMap<&str, HashSet<&str>>, HashMap<&str, usize>) {
    let mut candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_count = HashMap::new();

    for line in lines {
        let mut parts = line[0..line.len() - 1].split(" (contains ");

        let ingredients: HashSet<&str> = parts.next().unwrap().split_whitespace().collect();

        for ingredient in ingredients.clone() {
            *ingredient_count.entry(ingredient).or_insert(0) += 1;
        }

        let allergens: HashSet<&str> = parts.next().unwrap().split(", ").collect();

        for a in allergens {
            candidates
                .entry(a)
                .and_modify(|e| *e = (*e).intersection(&ingredients).copied().collect())
                .or_insert(ingredients.clone());
        }
    }
    (candidates, ingredient_count)
}

#[cfg(test)]
mod tests {
    use crate::day_21::{day_21_1, day_21_2};

    fn test_data() -> Vec<String> {
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_21_1() {
        assert_eq!(day_21_1(&test_data()), 5);
    }

    #[test]
    fn test_day_21_2() {
        assert_eq!(day_21_2(&test_data()), "mxmxvkd,sqjhc,fvjkl");
    }
}
