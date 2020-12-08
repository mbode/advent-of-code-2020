use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug)]
enum Command {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

pub fn day_8_1(lines: &[String]) -> isize {
    let commands: Vec<Command> = parse_commands(lines);

    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut i: isize = 0;

    loop {
        if visited.contains(&i) {
            break;
        }
        visited.insert(i);
        match commands.get(i as usize).unwrap() {
            Command::Acc(n) => {
                acc += n;
                i += 1;
            }
            Command::Jmp(n) => {
                i += *n;
            }
            Command::Nop(_) => {
                i += 1;
            }
        }
    }
    acc
}

fn parse_commands(lines: &[String]) -> Vec<Command> {
    lines
        .iter()
        .map(|l| {
            let mut split = l.split(' ');
            let command = split.next().unwrap();
            let number: isize = split.next().unwrap().parse().unwrap();
            match command {
                "acc" => Command::Acc(number),
                "jmp" => Command::Jmp(number),
                _ => Command::Nop(number),
            }
        })
        .collect()
}

pub fn day_8_2(lines: &[String]) -> isize {
    let mut jmps = HashSet::new();
    let mut nops = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        match line.split(' ').next().unwrap() {
            "jmp" => {
                jmps.insert(i as isize);
            }
            "nop" => {
                nops.insert(i as isize);
            }
            _ => {}
        }
    }

    let len = lines.len().try_into().unwrap();

    for jmp in jmps {
        let lines: Vec<String> = lines.iter().map(String::from).collect();
        let mut visited = HashSet::new();
        let mut acc = 0;
        let mut i: isize = 0;

        loop {
            if i < 0 || i >= len {
                break;
            }
            if visited.contains(&i) {
                break;
            }
            visited.insert(i);
            let line = lines.get(i as usize).unwrap();
            let mut split = line.split(' ');
            let command = split.next().unwrap();
            let number: isize = split.next().unwrap().parse().unwrap();

            match command {
                "acc" => {
                    acc += number;
                    i += 1;
                }
                "jmp" => {
                    if i == jmp {
                        i += 1;
                    } else {
                        i += number;
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        if i == len {
            return acc;
        }
    }
    for nop in nops {
        let lines: Vec<String> = lines.iter().map(String::from).collect();
        let mut visited = HashSet::new();
        let mut acc = 0;
        let mut i: isize = 0;

        loop {
            if i < 0 || i >= len {
                break;
            }
            if visited.contains(&i) {
                break;
            }
            visited.insert(i);
            let line = lines.get(i as usize).unwrap();
            let mut split = line.split(' ');
            let command = split.next().unwrap();
            let number: isize = split.next().unwrap().parse().unwrap();

            match command {
                "acc" => {
                    acc += number;
                    i += 1;
                }
                "jmp" => {
                    i += number;
                }
                _ => {
                    if i == nop {
                        i += number
                    } else {
                        i += 1;
                    }
                }
            }
        }
        if i == len {
            return acc;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_8::{day_8_1, day_8_2};

    fn test_data() -> Vec<String> {
        "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_8_1() {
        assert_eq!(day_8_1(&test_data()), 5);
    }

    #[test]
    fn test_day_8_2() {
        assert_eq!(day_8_2(&test_data()), 8);
    }
}
