use std::fmt;
use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    IoError,
    FromStrError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn read_input<T: FromStr>(year: u32, day: u32) -> Result<Vec<T>, Error> {
    let path = format!("data/{}/day{:02}.txt", year, day);

    let file = File::open(path).map_err(|_| Error::IoError)?;

    let input: Result<Vec<_>, _> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| T::from_str(l.trim()))
        .collect();

    match input {
        Ok(x) => Ok(x),
        Err(_) => Err(Error::FromStrError),
    }
}
