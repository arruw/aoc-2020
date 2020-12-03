use crate::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

type Input = Vec<PasswordEntry>;
type Output = usize;

#[allow(dead_code)]
fn input_generator(input: &str) -> Input {
    parse_input(input)
}

#[allow(dead_code)]
fn solve_part1(input: Input) -> Output {
    input
        .iter()
        .filter(|p| {
            let count = p.password.chars().filter(|c| c == &p.policy_char).count();

            p.policy_lower_bound <= count && count <= p.policy_upper_bound
        })
        .count()
}

#[allow(dead_code)]
fn solve_part2(input: Input) -> Output {
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

impl FromStr for PasswordEntry {
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
    use super::{input_generator, solve_part1, solve_part2};
    use crate::read_input;

    const DAY: u32 = 2;
    const SAMPLE: &str = " 1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";

    #[test]
    fn test_sample_part1() {
        let solution = solve_part1(input_generator(SAMPLE));

        assert_eq!(2, solution);
    }

    #[test]
    fn test_sample_part2() {
        let solution = solve_part2(input_generator(SAMPLE));

        assert_eq!(1, solution);
    }

    #[test]
    fn test_input_part1() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_generator(&input));

        assert_eq!(439, solution);
    }

    #[test]
    fn test_input_part2() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_generator(&input));

        assert_eq!(584, solution);
    }
}
