enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn left(&self, degrees: isize) -> Dir {
        match degrees {
            90 => match self {
                Dir::N => Dir::W,
                Dir::S => Dir::E,
                Dir::E => Dir::N,
                Dir::W => Dir::S,
            },
            180 => match self {
                Dir::N => Dir::S,
                Dir::S => Dir::N,
                Dir::E => Dir::W,
                Dir::W => Dir::E,
            },
            270 => match self {
                Dir::N => Dir::E,
                Dir::S => Dir::W,
                Dir::E => Dir::S,
                Dir::W => Dir::N,
            },
            _ => {
                panic!("dont know how to turn that way")
            }
        }
    }

    fn right(&self, degrees: isize) -> Dir {
        self.left(360 - degrees)
    }
}

pub fn day_12_1(lines: &[String]) -> isize {
    let mut ship = (0, 0);
    let mut dir = Dir::E;

    for line in lines {
        let mut chars = line.chars();
        let command = chars.next();
        let n = chars.collect::<String>().parse::<isize>().unwrap();
        match command {
            Some('N') => ship.1 += n,
            Some('S') => ship.1 -= n,
            Some('E') => ship.0 += n,
            Some('W') => ship.0 -= n,

            Some('L') => dir = dir.left(n),
            Some('R') => dir = dir.right(n),

            Some('F') => match dir {
                Dir::N => ship.1 += n,
                Dir::S => ship.1 -= n,
                Dir::E => ship.0 += n,
                Dir::W => ship.0 -= n,
            },
            _ => {}
        }
    }

    ship.0.abs() + ship.1.abs()
}

pub fn day_12_2(lines: &[String]) -> isize {
    let mut ship = (0, 0);

    let mut waypoint = (10, 1);

    for line in lines {
        let mut chars = line.chars();
        let command = chars.next();
        let n = chars.collect::<String>().parse::<isize>().unwrap();
        match command {
            Some('N') => waypoint.1 += n,
            Some('S') => waypoint.1 -= n,
            Some('E') => waypoint.0 += n,
            Some('W') => waypoint.0 -= n,

            Some('L') => waypoint = rotate(waypoint, n),
            Some('R') => waypoint = rotate(waypoint, -n),

            Some('F') => {
                ship.0 += n * waypoint.0;
                ship.1 += n * waypoint.1;
            }
            _ => {}
        }
    }

    ship.0.abs() + ship.1.abs()
}

fn rotate(point: (isize, isize), degrees: isize) -> (isize, isize) {
    let s = (degrees as f64).to_radians().sin() as isize;
    let c = (degrees as f64).to_radians().cos() as isize;

    (point.0 * c - point.1 * s, point.0 * s + point.1 * c)
}

#[cfg(test)]
mod tests {
    use crate::day_12::{day_12_1, day_12_2};

    fn test_data() -> Vec<String> {
        "F10
N3
F7
R90
F11"
        .lines()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_day_12_1() {
        assert_eq!(day_12_1(&test_data()), 25);
    }

    #[test]
    fn test_day_12_2() {
        assert_eq!(day_12_2(&test_data()), 286);
    }
}
