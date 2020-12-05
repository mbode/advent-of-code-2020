use std::collections::HashSet;

pub fn day_5_1(lines: &[String]) -> isize {
    lines.iter().map(|line| get_seat_id(line)).max().unwrap()
}

pub fn day_5_2(lines: &[String]) -> Option<isize> {
    let seat_ids: HashSet<isize> = lines.iter().map(|line| get_seat_id(line)).collect();

    (0..1023)
        .find(|i| {
            seat_ids.contains(&(i - 1)) && !seat_ids.contains(&i) && seat_ids.contains(&(i + 1))
        })
}

fn get_seat_id(line: &str) -> isize {
    get_row(String::from(&line[..7])) * 8 + get_column(String::from(&line[7..]))
}

fn get_row(row: String) -> isize {
    let s = row.replace("F", "0").replace("B", "1");
    isize::from_str_radix(s.as_str(), 2).unwrap()
}

fn get_column(column: String) -> isize {
    let s = column.replace("L", "0").replace("R", "1");
    isize::from_str_radix(s.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_5::{day_5_1, get_column, get_row};

    fn test_data() -> Vec<String> {
        vec![
            String::from("FBFBBFFRLR"),
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ]
    }

    #[test]
    fn test_get_row() {
        assert_eq!(get_row(String::from("FBFBBFF")), 44);
    }

    #[test]
    fn test_get_column() {
        assert_eq!(get_column(String::from("RLR")), 5);
    }

    #[test]
    fn test_day_5_1() {
        assert_eq!(day_5_1(&test_data()), 820);
    }
}
