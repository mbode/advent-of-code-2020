use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_isize(path: &str) -> Result<Vec<isize>, Error> {
    let br = BufReader::new(File::open(path)?);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_usize(path: &str) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(File::open(path)?);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_lines(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let br = BufReader::new(input);

    br.lines().collect()
}
