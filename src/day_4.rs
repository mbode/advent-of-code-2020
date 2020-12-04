use std::collections::HashSet;
use regex::Regex;

pub fn day_4_1(lines: &[String]) -> usize {
    let mut keys = HashSet::new();
    let mut valid = 0;
    for line in lines {
        if line == "" {
            if contains_all(&keys) {
                valid += 1;
            }
            keys = HashSet::new();
            continue;
        }
        for part in line.split(" ") {
            let key_value = part.split(":").collect::<Vec<&str>>();
            keys.insert(key_value[0]);
        }
    }
    if contains_all(&keys) {
        valid += 1;
    }
    valid
}

pub fn day_4_2(lines: &[String]) -> usize {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    let mut keys = HashSet::new();
    let mut valid = 0;
    let mut current = String::new();
    for line in lines {
        if line == "" {
            if contains_all(&keys) {
                valid += 1;
            }
            keys = HashSet::new();
            current = String::new();
            continue;
        }
        for part in line.split(" ") {
            let key_value = part.split(":").collect::<Vec<&str>>();
            let key = key_value[0];
            let value = key_value[1];
            match key {
                "byr" => {
                    if let Ok(n) = value.parse::<isize>() {
                        if n < 1920 || n > 2002 {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "iyr" => {
                    if let Ok(n) = value.parse::<isize>() {
                        if n < 2010 || n > 2020 {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "eyr" => {
                    if let Ok(n) = value.parse::<isize>() {
                        if n < 2020 || n > 2030 {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "hgt" => {
                    if value.ends_with("cm") {
                        if let Ok(n) = value[..value.len() - 2].parse::<isize>() {
                            if n < 150 || n > 193 {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else if value.ends_with("in") {
                        if let Ok(n) = value[..value.len() - 2].parse::<isize>() {
                            if n < 59 || n > 76 {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "hcl" => {
                    if !HCL_REGEX.is_match(value) {
                        continue;
                    }
                }
                "ecl" => {
                    let valid_values = vec!(
                        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
                    );
                    if !valid_values.contains(&value) {
                        continue;
                    }
                }
                "pid" => {
                    if !PID_REGEX.is_match(value) {
                        continue;
                    }
                }
                _ => continue,
            }
            keys.insert(key);
        }
        current += line;
        current += "";
    }
    if contains_all(&keys) {
        valid += 1;
    }
    valid
}

fn contains_all(keys: &HashSet<&str>) -> bool {
    keys.contains("byr") &&
        keys.contains("iyr") &&
        keys.contains("eyr") &&
        keys.contains("hgt") &&
        keys.contains("hcl") &&
        keys.contains("ecl") &&
        keys.contains("pid")
}

#[cfg(test)]
mod tests {
    use crate::day_4::{day_4_1, day_4_2};

    fn test_data() -> Vec<String> {
        vec!(
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        )
    }

    #[test]
    fn test_day_4_1() {
        assert_eq!(day_4_1(&test_data()), 2);
    }

    #[test]
    fn test_day_4_2_invalid() {
        assert_eq!(day_4_2(&vec!(
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
        )), 0);
    }

    #[test]
    fn test_day_4_2_valid() {
        assert_eq!(day_4_2(&vec!(
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        )), 4);
    }
}