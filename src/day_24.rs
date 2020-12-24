use hex_grid::{Coordinate, Offset, CENTER, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP_LEFT, UP_RIGHT};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct Tile {
    color: Color,
}

impl Tile {
    fn flip(&mut self) {
        self.color = match &self.color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub fn day_24_1(lines: &[String]) -> usize {
    let grid = prepare_tiles(lines);

    grid.values()
        .filter(|&tile| tile.color == Color::Black)
        .count()
}

pub fn day_24_2(lines: &[String]) -> usize {
    let mut grid = prepare_tiles(lines);

    for _ in 0..100 {
        let previous = grid.clone();
        for (&c, &t) in &previous {
            let black = (c + Offset::fill_hex(1))
                .iter()
                .filter(|&&coord| coord != c)
                .filter(|&coord| {
                    if let Some(tile) = previous.get(coord) {
                        return tile.color == Color::Black;
                    }
                    false
                })
                .count();
            match t.color {
                Color::White => {
                    if black == 2 {
                        grid.entry(c).and_modify(|tile| tile.flip());
                    }
                }
                Color::Black => {
                    if black == 0 || black > 2 {
                        grid.entry(c).and_modify(|tile| tile.flip());
                    }
                }
            }
        }
    }

    grid.values()
        .filter(|&tile| tile.color == Color::Black)
        .count()
}

fn prepare_tiles(lines: &[String]) -> HashMap<Coordinate, Tile> {
    let directions: HashMap<&str, Offset> = HashMap::from_iter(vec![
        ("e", RIGHT),
        ("se", DOWN_RIGHT),
        ("sw", DOWN_LEFT),
        ("w", LEFT),
        ("nw", UP_LEFT),
        ("ne", UP_RIGHT),
    ]);

    let mut to_flip = Vec::new();
    for line in lines {
        let mut rest: &str = line;
        let mut coordinate = CENTER;

        while !rest.is_empty() {
            for (&s, &offset) in &directions {
                if let Some(new_rest) = rest.strip_prefix(s) {
                    coordinate = coordinate + offset;
                    rest = new_rest;
                }
            }
        }
        to_flip.push(coordinate);
    }

    let mut grid: HashMap<Coordinate, Tile> = HashMap::new();

    for coord in CENTER + Offset::fill_hex(100) {
        grid.insert(
            coord,
            Tile {
                color: Color::White,
            },
        );
    }

    for coordinate in to_flip {
        grid.entry(coordinate).and_modify(|tile| tile.flip());
    }
    grid
}

#[cfg(test)]
mod tests {
    use crate::day_24::{day_24_1, day_24_2};

    fn test_data() -> Vec<String> {
        "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_day_24_1() {
        assert_eq!(day_24_1(&test_data()), 10);
    }

    #[test]
    fn test_day_24_2() {
        assert_eq!(day_24_2(&test_data()), 2208);
    }
}
