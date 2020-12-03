use crate::utils::*;

use regex::Regex;
use std::error::Error;
use std::str::FromStr;

type Input = Vec<PasswordEntry>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    parse_input(input)
}

fn solve_part1(input: Input) -> Output {
    input
        .iter()
        .filter(|p| {
            let count = p.password.chars().filter(|c| c == &p.policy_char).count();

            p.policy_lower_bound <= count && count <= p.policy_upper_bound
        })
        .count()
}

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
    use super::{input_transformer, solve_part1, solve_part2};
    use crate::utils::*;

    const DAY: u32 = 2;
    const SAMPLE: &str = " 1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(2, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(439, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(1, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(584, solution);
    }
}
