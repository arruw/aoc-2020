use std::io;
use std::str::FromStr;
use std::{error::Error, iter::FromIterator};
use std::{fmt::Debug, fs, path::Path};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn read_input(year: u32, day: u32) -> io::Result<String> {
    let path = format!("input/{}/day{}.txt", year, day);

    fs::read_to_string(Path::new(&path))
}

pub fn parse_input<C, T>(input: &str) -> C
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    C: FromIterator<T>,
{
    let input = input
        .lines()
        .map(|l| l.trim().to_string())
        .map(|l| l.parse::<T>().unwrap())
        .collect();

    input
}
