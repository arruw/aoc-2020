use std::error::Error;

use aoc::read_input;
use aoc::Result;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    println!("Day 2!");
    let mut input: Vec<PasswordEntry> = read_input(2020, 2).unwrap();
    let solution = solve_p2(&mut input);
    println!("Solution: {}", solution);

    Ok(())
}

fn solve_p1(input: &mut Vec<PasswordEntry>) -> usize {
    input
        .iter()
        .filter(|p| {
            let count = p.password.chars().filter(|c| c == &p.policy_char).count();

            p.policy_lower_bound <= count && count <= p.policy_upper_bound
        })
        .count()
}

fn solve_p2(input: &mut Vec<PasswordEntry>) -> usize {
    input
        .iter()
        .filter(|p| {
            let c1 = p.password.chars().nth(p.policy_lower_bound - 1).unwrap();
            let c2 = p.password.chars().nth(p.policy_upper_bound - 1).unwrap();

            (c1 == p.policy_char) ^ (c2 == p.policy_char)
        })
        .count()
}

struct PasswordEntry {
    password: String,
    policy_char: char,
    policy_lower_bound: usize,
    policy_upper_bound: usize,
}

impl std::str::FromStr for PasswordEntry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<lb>\d+)-(?P<ub>\d+) (?P<c>\w): (?P<pwd>\w+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Ok(PasswordEntry {
            password: cap.name("pwd").unwrap().as_str().parse()?,
            policy_char: cap.name("c").unwrap().as_str().parse()?,
            policy_lower_bound: cap.name("lb").unwrap().as_str().parse()?,
            policy_upper_bound: cap.name("ub").unwrap().as_str().parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample_input() {
        let input = &mut vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|l| l.parse::<PasswordEntry>().unwrap())
            .collect();
        let solution = solve_p1(input);

        assert_eq!(2, solution);
    }

    #[test]
    fn test_part2_sample_input() {
        let input = &mut vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|l| l.parse::<PasswordEntry>().unwrap())
            .collect();
        let solution = solve_p2(input);

        assert_eq!(1, solution);
    }

    #[test]
    fn test_part1_solution() {
        let mut input: Vec<PasswordEntry> = read_input(2020, 2).unwrap();
        let solution = solve_p1(&mut input);
        assert_eq!(439, solution);
    }

    #[test]
    fn test_part2_solution() {
        let mut input: Vec<PasswordEntry> = read_input(2020, 2).unwrap();
        let solution = solve_p2(&mut input);
        assert_eq!(584, solution);
    }
}
