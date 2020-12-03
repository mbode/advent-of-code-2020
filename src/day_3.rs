pub fn day_3_1(lines: &[String], right: usize, down: usize) -> usize {
    let mut row = 0;
    let mut count = 0;
    for (i, line) in lines.iter().enumerate() {
        if i % down == 0 {
            if line.chars().nth(row).unwrap() == '#' {
                count += 1;
            }
            row += right;
            row %= line.len()
        }
    }
    count
}

pub fn day_3_2(lines: &[String]) -> usize {
    vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
        .iter()
        .map(|(right, down)| day_3_1(lines, *right, *down))
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day_3::{day_3_1, day_3_2};

    fn test_data() -> Vec<String> {
        vec!(
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        )
    }

    #[test]
    fn test_day_3_1() {
        assert_eq!(day_3_1(&test_data(), 3, 1), 7);
    }

    #[test]
    fn test_day_3_2() {
        assert_eq!(day_3_2(&test_data()), 336);
    }
}