use std::io;
use std::iter::FromIterator;
use std::str::FromStr;
use std::{fmt::Debug, fs, path::Path};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
        .map(|l| l.trim().parse::<T>().unwrap())
        .collect();

    input
}

pub trait CharAtExt {
    fn char_at(&self, at: usize) -> Option<char>;
}

impl CharAtExt for String {
    fn char_at(&self, at: usize) -> Option<char> {
        self.chars().nth(at)
    }
}

impl CharAtExt for &str {
    fn char_at(&self, at: usize) -> Option<char> {
        self.chars().nth(at)
    }
}

pub fn modulo(x: isize, m: usize) -> usize {
    let m = m as isize;
    ((x % m + m) % m) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo() {
        assert_eq!(2, modulo(2, 10));
        assert_eq!(0, modulo(10, 10));
        assert_eq!(8, modulo(-2, 10));
        assert_eq!(8, modulo(-12, 10));
    }
}
