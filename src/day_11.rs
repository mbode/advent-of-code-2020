pub fn day_11_1(lines: &[String]) -> usize {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().into_iter().collect());
    }
    let number_of_lines = matrix.len();
    let line_length = matrix[0].len();
    loop {
        let mut changed = false;
        let previous = matrix.clone();
        for i in 0..number_of_lines {
            for j in 0..line_length {
                match previous[i][j] {
                    '.' => {}
                    '#' => {
                        if count_adjacent(&previous, i, j, number_of_lines, line_length) >= 4 {
                            changed = true;
                            matrix[i][j] = 'L';
                        }
                    }
                    'L' => {
                        if count_adjacent(&previous, i, j, number_of_lines, line_length) == 0 {
                            changed = true;
                            matrix[i][j] = '#';
                        }
                    }
                    _ => {}
                }
            }
        }

        if !changed {
            break;
        }
    }
    matrix
        .into_iter()
        .map(|l| l.into_iter().filter(|&c| c == '#').count())
        .sum()
}

fn count_adjacent(
    previous: &[Vec<char>],
    i: usize,
    j: usize,
    number_of_lines: usize,
    line_length: usize,
) -> i32 {
    let mut adjacent = 0;
    for i_dir in -1..2 {
        for j_dir in -1..2 {
            if i_dir != 0 || j_dir != 0 {
                let x: isize = i as isize + i_dir;
                let y: isize = j as isize + j_dir;
                if x >= 0 && x < number_of_lines as isize && y >= 0 && y < line_length as isize {
                    if let '#' = previous[x as usize][y as usize] {
                        adjacent += 1;
                    }
                }
            }
        }
    }
    adjacent
}

pub fn day_11_2(lines: &[String]) -> usize {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().into_iter().collect());
    }
    let number_of_lines = matrix.len();
    let line_length = matrix[0].len();
    loop {
        let mut changed = false;
        let previous = matrix.clone();
        for i in 0..number_of_lines {
            for j in 0..line_length {
                match previous[i][j] {
                    '.' => {}
                    '#' => {
                        if count_in_line_of_sight(&previous, i, j, number_of_lines, line_length)
                            >= 5
                        {
                            changed = true;
                            matrix[i][j] = 'L';
                        }
                    }
                    'L' => {
                        if count_in_line_of_sight(&previous, i, j, number_of_lines, line_length)
                            == 0
                        {
                            changed = true;
                            matrix[i][j] = '#';
                        }
                    }
                    _ => {}
                }
            }
        }

        if !changed {
            break;
        }
    }
    matrix
        .into_iter()
        .map(|l| l.into_iter().filter(|&c| c == '#').count())
        .sum()
}

fn count_in_line_of_sight(
    previous: &[Vec<char>],
    i: usize,
    j: usize,
    number_of_lines: usize,
    line_length: usize,
) -> i32 {
    let mut in_line_of_sight = 0;
    for i_dir in -1..2 {
        for j_dir in -1..2 {
            if i_dir != 0 || j_dir != 0 {
                let mut x: isize = i as isize + i_dir;
                let mut y: isize = j as isize + j_dir;
                while x >= 0 && x < number_of_lines as isize && y >= 0 && y < line_length as isize {
                    match previous[x as usize][y as usize] {
                        '#' => {
                            in_line_of_sight += 1;
                            break;
                        }
                        'L' => break,
                        _ => {}
                    }
                    x += i_dir;
                    y += j_dir;
                }
            }
        }
    }
    in_line_of_sight
}

#[cfg(test)]
mod tests {
    use crate::day_11::{day_11_1, day_11_2};

    fn test_data() -> Vec<String> {
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_11_1() {
        assert_eq!(day_11_1(&test_data()), 37);
    }

    #[test]
    fn test_day_11_2() {
        assert_eq!(day_11_2(&test_data()), 26);
    }
}
