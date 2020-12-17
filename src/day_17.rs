use itertools::iproduct;
use std::collections::HashMap;

pub fn day_17_1(lines: &[String]) -> usize {
    let mut grid: HashMap<(isize, isize, isize), bool> = HashMap::new();
    let number_of_lines = lines.len();
    let line_len = lines[0].len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                (
                    x as isize - (line_len as isize) / 2,
                    y as isize - (number_of_lines as isize) / 2,
                    0,
                ),
                c == '#',
            );
        }
    }

    for _ in 1..7 {
        let min_x = grid.keys().min_by_key(|(x, _, _)| x).unwrap().0;
        let previous = grid.clone();
        for (x, y, z) in iproduct!(
            min_x - 1..-min_x + 2,
            min_x - 1..-min_x + 2,
            min_x - 1..-min_x + 2
        ) {
            let neighbors = iproduct!(x - 1..x + 2, y - 1..y + 2, z - 1..z + 2)
                .filter(|t| *t != (x, y, z) && previous.get(t) == Some(&true))
                .count();

            match previous.get(&(x, y, z)) {
                Some(true) => {
                    grid.insert((x, y, z), (2..4).contains(&neighbors));
                }
                Some(false) | None => {
                    grid.insert((x, y, z), neighbors == 3);
                }
            }
        }
    }

    grid.values().filter(|&&b| b).count()
}

pub fn day_17_2(lines: &[String]) -> usize {
    let mut grid: HashMap<(isize, isize, isize, isize), bool> = HashMap::new();
    let number_of_lines = lines.len();
    let line_len = lines[0].len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                (
                    x as isize - (line_len as isize) / 2,
                    y as isize - (number_of_lines as isize) / 2,
                    0,
                    0,
                ),
                c == '#',
            );
        }
    }

    for _ in 1..7 {
        let min_x = grid.keys().min_by_key(|(x, _, _, _)| x).unwrap().0;
        let previous = grid.clone();
        for (x, y, z, w) in iproduct!(
            min_x - 1..-min_x + 2,
            min_x - 1..-min_x + 2,
            min_x - 1..-min_x + 2,
            min_x - 1..-min_x + 2
        ) {
            let neighbors = iproduct!(x - 1..x + 2, y - 1..y + 2, z - 1..z + 2, w - 1..w + 2)
                .filter(|t| *t != (x, y, z, w) && previous.get(t) == Some(&true))
                .count();

            match previous.get(&(x, y, z, w)) {
                Some(true) => {
                    grid.insert((x, y, z, w), (2..4).contains(&neighbors));
                }
                Some(false) | None => {
                    grid.insert((x, y, z, w), neighbors == 3);
                }
            }
        }
    }

    grid.values().filter(|&&b| b).count()
}

#[cfg(test)]
mod tests {
    use crate::day_17::{day_17_1, day_17_2};

    fn test_data() -> Vec<String> {
        ".#.
..#
###"
        .lines()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_day_17_1() {
        assert_eq!(day_17_1(&test_data()), 112);
    }

    #[test]
    fn test_day_17_2() {
        assert_eq!(day_17_2(&test_data()), 848);
    }
}
