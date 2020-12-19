use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RulePart {
    Char(char),
    Other(usize),
}

pub fn day_19_1(lines: &[String]) -> usize {
    let mut sections = lines.split(|l| l == "");

    let mut parsed_rules = parse_rules(sections.next().unwrap());
    let rules = compute(&mut parsed_rules);

    let rule_0 = rules.get(&0).unwrap();
    sections
        .next()
        .unwrap()
        .iter()
        .filter(|&l| rule_0.contains(l))
        .count()
}

pub fn day_19_2(lines: &[String]) -> usize {
    let mut sections = lines.split(|l| l == "");

    let mut parsed_rules = parse_rules(sections.next().unwrap());
    let rules = compute(&mut parsed_rules);

    //  0: 8 11
    //  8: 42 | 42 8  => n times 42
    // 11: 42 31 | 42 11 31  => m times 42, 11, 31 => m times 42, m times 31
    let rule_42 = rules.get(&42).unwrap();
    let rule_31 = rules.get(&31).unwrap();
    sections
        .next()
        .unwrap()
        .iter()
        .filter(|&l| check_starts_with(l, rule_42, rule_31))
        .count()
}

fn parse_rules(lines: &[String]) -> HashMap<usize, HashSet<Vec<RulePart>>> {
    let mut parsed_rules: HashMap<usize, HashSet<Vec<RulePart>>> = HashMap::new();

    for line in lines {
        let mut parts = line.split(": ");
        let rule: usize = parts.next().unwrap().parse().unwrap();
        let possibiliites = parts.next().unwrap().split(" | ");
        let mut set: HashSet<Vec<RulePart>> = HashSet::new();
        for p in possibiliites {
            let mut choices: Vec<RulePart> = Vec::new();
            for c in p.split_whitespace() {
                if let Ok(n) = c.parse::<usize>() {
                    choices.push(RulePart::Other(n));
                } else {
                    choices.push(RulePart::Char(c.chars().nth(1).unwrap()));
                }
            }
            set.insert(choices);
        }
        parsed_rules.insert(rule, set);
    }
    parsed_rules
}

fn compute(
    parsed_rules: &mut HashMap<usize, HashSet<Vec<RulePart>>>,
) -> HashMap<usize, HashSet<String>> {
    let mut result = HashMap::new();
    while !parsed_rules.is_empty() {
        for (rule, choices) in parsed_rules.clone() {
            let only_chars: fn(&Vec<RulePart>) -> bool = |v| {
                v.iter()
                    .all(|rule_part| matches!(rule_part, RulePart::Char(_)))
            };
            if choices.iter().all(only_chars) {
                let mut strings = HashSet::new();
                for choice in choices.iter() {
                    let s: String = choice
                        .iter()
                        .map(|rule_part| {
                            if let RulePart::Char(c) = rule_part {
                                c
                            } else {
                                unreachable!()
                            }
                        })
                        .collect();
                    strings.insert(s);
                }
                result.insert(rule, strings);
                parsed_rules.remove(&rule);
            }
        }
        for (rule, choices) in parsed_rules.clone() {
            let mut new_choices = HashSet::new();
            for c in choices.iter() {
                let mut new_choices_here = HashSet::new();
                new_choices_here.insert(Vec::new());
                for rule_part in c {
                    let previous = new_choices_here.clone();
                    new_choices_here = HashSet::new();
                    if let RulePart::Other(n) = rule_part {
                        if let Some(set) = result.get(&n) {
                            for nc in previous {
                                for s in set {
                                    let mut new = nc.clone();
                                    s.chars().for_each(|c| new.push(RulePart::Char(c)));
                                    new_choices_here.insert(new);
                                }
                            }
                        } else {
                            for nc in previous {
                                let mut new = nc;
                                new.push(*rule_part);
                                new_choices_here.insert(new);
                            }
                        }
                    } else {
                        for nc in previous {
                            let mut new = nc;
                            new.push(*rule_part);
                            new_choices_here.insert(new);
                        }
                    }
                }
                for new in new_choices_here {
                    new_choices.insert(new);
                }
            }
            parsed_rules.insert(rule, new_choices);
        }
    }

    result
}

fn check_starts_with(
    input: &str,
    starts_with: &HashSet<String>,
    ends_with: &HashSet<String>,
) -> bool {
    for prefix in starts_with {
        if input.starts_with(prefix) {
            let new_input = &input[prefix.len()..];
            return check_starts_with(new_input, starts_with, ends_with)
                || check_starts_with_ends_with(new_input, starts_with, ends_with);
        }
    }
    false
}

fn check_starts_with_ends_with(
    input: &str,
    starts_with: &HashSet<String>,
    ends_with: &HashSet<String>,
) -> bool {
    for prefix in starts_with {
        for suffix in ends_with {
            if input.starts_with(prefix) && input.ends_with(suffix) {
                let p_len = prefix.len();
                let s_len = suffix.len();
                if input.len() == p_len + s_len {
                    return true;
                } else {
                    return check_starts_with_ends_with(
                        &input[p_len..input.len() - s_len],
                        starts_with,
                        ends_with,
                    );
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day_19::{day_19_1, day_19_2};

    fn test_data() -> Vec<String> {
        "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            .lines()
            .map(String::from)
            .collect()
    }

    fn test_data_2() -> Vec<String> {
        "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_19_1() {
        assert_eq!(day_19_1(&test_data()), 2);
    }

    #[test]
    fn test_day_19_2() {
        assert_eq!(day_19_2(&test_data_2()), 12);
    }
}
