use std::collections::{HashMap, HashSet};

const MONSTER: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    n: usize,
    borders: HashMap<Direction, Vec<bool>>,
    inner: Vec<Vec<bool>>,
}

impl Tile {
    fn rotate(&self) -> Tile {
        let mut new_borders = HashMap::new();
        new_borders.insert(Direction::Up, self.borders[&Direction::Right].clone());
        let mut right = self.borders[&Direction::Down].clone();
        right.reverse();
        new_borders.insert(Direction::Right, right);
        new_borders.insert(Direction::Down, self.borders[&Direction::Left].clone());
        let mut up = self.borders[&Direction::Up].clone();
        up.reverse();
        new_borders.insert(Direction::Left, up);

        Tile {
            n: self.n,
            borders: new_borders,
            inner: rotate(&self.inner),
        }
    }

    fn flip(&self) -> Tile {
        let mut new_borders = HashMap::new();
        let mut up = self.borders[&Direction::Up].clone();
        up.reverse();
        new_borders.insert(Direction::Up, up);
        new_borders.insert(Direction::Right, self.borders[&Direction::Left].clone());
        let mut down = self.borders[&Direction::Down].clone();
        down.reverse();
        new_borders.insert(Direction::Down, down);
        new_borders.insert(Direction::Left, self.borders[&Direction::Right].clone());

        Tile {
            n: self.n,
            borders: new_borders,
            inner: flip(&self.inner),
        }
    }
}

pub fn day_20_1(lines: &[String]) -> usize {
    let tiles = parse_tiles(lines);

    let mut prod = 1;
    for (n, tile) in &tiles {
        let mut not_matched = 0;
        for border in tile.borders.values() {
            let mut other_borders = HashSet::new();
            for other in tiles.values() {
                if other.n != *n {
                    other_borders.extend(other.borders.values());
                }
            }
            let mut reversed: Vec<bool> = border.clone();
            reversed.reverse();
            if !other_borders.contains(&border) && !other_borders.contains(&reversed) {
                not_matched += 1;
            }
        }
        if not_matched == 2 {
            prod *= n;
        }
    }

    prod
}

fn parse_tiles(lines: &[String]) -> HashMap<usize, Tile> {
    let mut tiles = HashMap::new();

    for tile in lines.split(|l| l == "") {
        let mut lines_in_tile = tile.iter();
        let n: usize = lines_in_tile
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .split(':')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut left: Vec<bool> = Vec::new();
        let mut right: Vec<bool> = Vec::new();
        let mut borders = HashMap::new();
        let mut inner: Vec<Vec<bool>> = Vec::new();
        for (i, line) in lines_in_tile.enumerate() {
            if i == 0 {
                borders.insert(Direction::Up, line.chars().map(|c| c == '#').collect());
            }
            if i == tile.len() - 2 {
                borders.insert(Direction::Down, line.chars().map(|c| c == '#').collect());
            }
            left.push(line.starts_with('#'));
            right.push(line.ends_with('#'));
            let inner_line: Vec<bool> = line
                .chars()
                .skip(1)
                .take(line.len() - 2)
                .map(|c| c == '#')
                .collect();
            if i > 0 && i < 9 {
                inner.push(inner_line);
            }
        }
        borders.insert(Direction::Left, left);
        borders.insert(Direction::Right, right);

        tiles.insert(n, Tile { n, inner, borders });
    }
    tiles
}

fn rotate(vec_vec: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let line_length = vec_vec.iter().next().unwrap().len();
    let mut lines = Vec::new();
    for j in (0..line_length).rev() {
        let mut new_line = Vec::new();
        for line in vec_vec {
            new_line.push(line[j]);
        }
        lines.push(new_line);
    }
    lines
}

fn flip(vec_vec: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut lines = Vec::new();
    for line in vec_vec {
        let mut new_line = line.clone();
        new_line.reverse();
        lines.push(new_line);
    }
    lines
}

pub fn day_20_2(lines: &[String], first_corner: usize) -> usize {
    let tiles = parse_tiles(lines);

    let mut all_tiles = Vec::new();
    for tile in tiles.values() {
        all_tiles.push(tile.clone());
        all_tiles.push(tile.rotate());
        all_tiles.push(tile.rotate().rotate());
        all_tiles.push(tile.rotate().rotate().rotate());
        all_tiles.push(tile.flip());
        all_tiles.push(tile.rotate().flip());
        all_tiles.push(tile.rotate().rotate().flip());
        all_tiles.push(tile.rotate().rotate().rotate().flip());
    }

    let mut col: Vec<Tile> = Vec::new();
    col.push(tiles[&first_corner].flip().rotate().rotate());

    loop {
        let last = col.iter().last().unwrap();
        match all_tiles
            .iter()
            .find(|t| t.borders[&Direction::Up] == last.borders[&Direction::Down] && t.n != last.n)
        {
            None => {
                break;
            }
            Some(tile) => {
                col.push(tile.clone());
            }
        }
    }
    let mut image: Vec<Vec<bool>> = Vec::new();
    for t in col {
        let mut row: Vec<Tile> = Vec::new();
        row.push(t);
        loop {
            let last = row.iter().last().unwrap();
            match all_tiles.iter().find(|t| {
                t.borders[&Direction::Left] == last.borders[&Direction::Right] && t.n != last.n
            }) {
                None => {
                    break;
                }
                Some(tile) => {
                    row.push(tile.clone());
                }
            }
        }
        let mut lines = Vec::new();
        for _ in 0..8 {
            lines.push(Vec::new());
        }
        for tile in row {
            for (j, line) in tile.inner.iter().enumerate() {
                lines[j].extend(line);
            }
        }
        image.extend(lines);
    }

    let monsters_found = find_monsters(&image)
        + find_monsters(&rotate(&image))
        + find_monsters(&rotate(&rotate(&image)))
        + find_monsters(&rotate(&rotate(&rotate(&image))))
        + find_monsters(&flip(&image))
        + find_monsters(&flip(&rotate(&image)))
        + find_monsters(&flip(&rotate(&rotate(&image))))
        + find_monsters(&flip(&rotate(&rotate(&rotate(&image)))));

    let monster_size = MONSTER.matches('#').count();

    let total_size: usize = tiles
        .values()
        .into_iter()
        .map(|t| {
            t.inner
                .iter()
                .map(|v| v.iter().filter(|&b| *b).count())
                .sum::<usize>()
        })
        .sum();

    total_size - monsters_found * monster_size
}

fn find_monsters(image: &[Vec<bool>]) -> usize {
    let mut monster = Vec::new();
    for line in MONSTER.lines() {
        monster.push(line.chars().map(|c| c == '#').collect::<Vec<bool>>());
    }

    let mut monsters_found = 0;

    for i in 0..image.len() - monster.len() {
        'image: for j in 0..image[0].len() - monster[0].len() {
            for k in 0..monster.len() {
                for l in 0..monster[0].len() {
                    if monster[k][l] && !image[i + k][j + l] {
                        continue 'image;
                    }
                }
            }
            monsters_found += 1;
        }
    }

    monsters_found
}

fn _print_field(vec_vec: Vec<Vec<bool>>) {
    for row in vec_vec {
        println!(
            "{}",
            row.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::day_20::{day_20_1, day_20_2};

    fn test_data() -> Vec<String> {
        "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"
        .lines()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_day_20_1() {
        assert_eq!(day_20_1(&test_data()), 20899048083289);
    }

    #[test]
    fn test_day_20_2() {
        assert_eq!(day_20_2(&test_data(), 1951), 273);
    }
}
