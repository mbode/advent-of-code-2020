pub fn day_25(lines: &[String]) -> usize {
    let card_key: usize = lines[0].parse().unwrap();
    let door_key: usize = lines[1].parse().unwrap();

    let subject = 7;
    let mut card_loop_size = 0;
    let mut current = subject;
    while current != card_key {
        card_loop_size += 1;
        current = (current * subject) % 20201227;
    }

    let subject = door_key;
    let mut encryption_key = door_key;
    for _ in 0..card_loop_size {
        encryption_key = (encryption_key * subject) % 20201227;
    }
    encryption_key
}

#[cfg(test)]
mod tests {
    use crate::day_25::day_25;

    fn test_data() -> Vec<String> {
        "5764801
17807724"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_25() {
        assert_eq!(day_25(&test_data()), 14897079);
    }
}
