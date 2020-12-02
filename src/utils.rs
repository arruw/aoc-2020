use std::error::Error;
use std::fmt::Debug;
use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

pub fn read_input<T>(year: u32, day: u32) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let path = format!("data/{}/day{:02}.txt", year, day);

    let file = File::open(path).expect("Can not open the input file!");

    let input: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Can not read input line!"))
        .map(|l| l.trim().to_string())
        .map(|l| l.parse::<T>().expect("Can not parse the input line!"))
        .collect();

    Ok(input)
}
