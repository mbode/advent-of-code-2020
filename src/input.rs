use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_isize_per_line(path: &str) -> Result<Vec<isize>, Error> {
    let br = BufReader::new(File::open(path)?);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_usize_per_line(path: &str) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(File::open(path)?);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_usize_one_line(path: &str) -> Result<Vec<usize>, Error> {
    read_lines(path)?
        .get(0)
        .unwrap()
        .split(',')
        .map(|i| {
            i.parse::<usize>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        })
        .collect()
}

pub fn read_lines(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let br = BufReader::new(input);

    br.lines().collect()
}
