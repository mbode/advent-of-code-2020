#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Plus,
    Times,
    ParenOpen,
    ParenClose,
    N(isize),
}

pub fn day_18_1(lines: &[String]) -> isize {
    lines
        .iter()
        .map(parse_commands)
        .map(|v| reduce_parens(&v, compute_1))
        .map(compute_1)
        .sum()
}

pub fn day_18_2(lines: &[String]) -> isize {
    lines
        .iter()
        .map(parse_commands)
        .map(|v| reduce_parens(&v, compute_2))
        .map(compute_2)
        .sum()
}

fn parse_commands(line: &String) -> Vec<Operation> {
    let mut commands = Vec::new();

    for part in line.split_whitespace() {
        match part {
            "+" => commands.push(Operation::Plus),
            "*" => commands.push(Operation::Times),
            _ => {
                commands = parse_single(commands, part);
            }
        }
    }
    commands
}

fn parse_single(commands: Vec<Operation>, part: &str) -> Vec<Operation> {
    let mut commands = commands;
    if let Some(rest) = part.strip_prefix('(') {
        commands.push(Operation::ParenOpen);
        commands = parse_single(commands, rest)
    } else if let Some(rest) = part.strip_suffix(')') {
        commands = parse_single(commands, rest);
        commands.push(Operation::ParenClose);
    } else if let Ok(n) = part.parse::<isize>() {
        commands.push(Operation::N(n));
    }
    commands
}

fn reduce_parens(
    commands: &[Operation],
    compute_fn: fn(Vec<Operation>) -> isize,
) -> Vec<Operation> {
    let mut commands = commands.to_owned();
    while commands.contains(&Operation::ParenOpen) {
        let previous = commands.to_owned();
        let last_open = commands
            .iter()
            .enumerate()
            .rfind(|(_, &o)| o == Operation::ParenOpen)
            .unwrap()
            .0;

        let next_close = commands
            .iter()
            .enumerate()
            .find(|(i, &o)| *i > last_open && o == Operation::ParenClose)
            .unwrap()
            .0;
        commands = Vec::new();
        for op in previous.iter().take(last_open) {
            commands.push(*op);
        }
        commands.push(Operation::N(compute_fn(
            previous[last_open + 1..next_close].to_vec(),
        )));
        for op in previous.iter().skip(next_close + 1) {
            commands.push(*op);
        }
    }
    commands
}

fn compute_1(commands: Vec<Operation>) -> isize {
    let mut result = 0;
    let mut op: Option<Operation> = None;
    for command in commands {
        match command {
            Operation::Plus => op = Some(Operation::Plus),
            Operation::Times => op = Some(Operation::Times),
            Operation::N(n) => match op {
                None => result = n,
                Some(Operation::Plus) => result += n,
                Some(Operation::Times) => result *= n,
                _ => (),
            },
            _ => (),
        }
    }
    result
}

fn compute_2(commands: Vec<Operation>) -> isize {
    let mut commands = commands;
    while commands.contains(&Operation::Plus) {
        let previous = commands.clone();
        let first_plus = commands
            .iter()
            .enumerate()
            .find(|(_, &o)| o == Operation::Plus)
            .unwrap()
            .0;

        commands = Vec::new();
        for op in previous.iter().take(first_plus - 1) {
            commands.push(*op);
        }
        if let Operation::N(i) = previous[first_plus - 1] {
            if let Operation::N(j) = previous[first_plus + 1] {
                commands.push(Operation::N(i + j))
            }
        }
        for op in previous.iter().skip(first_plus + 2) {
            commands.push(*op);
        }
    }

    compute_1(commands)
}

#[cfg(test)]
mod tests {
    use crate::day_18::{day_18_1, day_18_2};

    fn test_data() -> Vec<String> {
        "1 + 2 * 3 + 4 * 5 + 6".lines().map(String::from).collect()
    }

    #[test]
    fn test_day_18_1() {
        assert_eq!(day_18_1(&test_data()), 71);
    }

    #[test]
    fn test_day_18_1_paren() {
        assert_eq!(
            day_18_1(&vec!(String::from(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
            ))),
            12240
        );
    }

    #[test]
    fn test_day_18_2() {
        assert_eq!(day_18_2(&test_data()), 231);
    }

    #[test]
    fn test_day_18_2_b() {
        assert_eq!(
            day_18_2(&vec!(String::from("1 + (2 * 3) + (4 * (5 + 6))"))),
            51
        );
    }

    #[test]
    fn test_day_18_2_c() {
        assert_eq!(day_18_2(&vec!(String::from("2 * 3 + (4 * 5)"))), 46);
    }

    #[test]
    fn test_day_18_2_d() {
        assert_eq!(
            day_18_2(&vec!(String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"))),
            1445
        );
    }
}
