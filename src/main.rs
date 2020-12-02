mod day_1;

use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() -> Result<(), Error> {
    let numbers = read_input("input/day_1")?;

    if let Some(i) = day_1::day_1_1(&numbers) {
        println! {"Two: {}", i}
    }
    if let Some(i) = day_1::day_1_2(&numbers) {
        println! {"Three: {}", i}
    }
    Ok(())
}

fn read_input(path: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(path)?;
    let br = BufReader::new(input);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}