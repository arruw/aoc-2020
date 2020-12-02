use std::fmt::Debug;
use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use std::{error::Error, iter::FromIterator};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn read_input<C, T>(year: u32, day: u32) -> Result<C>
where
    T: FromStr,
    T: Sized,
    <T as FromStr>::Err: Debug,
    C: FromIterator<T>,
{
    let path = format!("data/{}/day{:02}.txt", year, day);

    let file = File::open(path).expect("Can not open the input file!");

    let input: C = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Can not read input line!"))
        .map(|l| l.trim().to_string())
        .map(|l| l.parse::<T>().expect("Can not parse the input line!"))
        .collect();

    Ok(input)
}
