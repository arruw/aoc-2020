use std::error::Error;

use aoc::read_input;
use itertools::Itertools;
use std::result::Result;

fn main() {
    println!("Day 2!");
    let mut input = read_input::<PasswordEntry<PasswordPolicy2>>(2020, 2).unwrap();
    let solution = solve(&mut input);
    println!("Solution: {}", solution);
}

fn solve<T>(input: &mut Vec<PasswordEntry<T>>) -> usize
where
    T: PasswordPolicyValidator,
{
    input
        .iter()
        .filter(|p| p.policy.is_policy_valid(&p.password))
        .count()
}

struct PasswordEntry<T>
where
    T: PasswordPolicyValidator,
{
    password: String,
    policy: T,
}

trait PasswordPolicyValidator {
    fn is_policy_valid(&self, password: &str) -> bool;
}

struct PasswordPolicy1 {
    char: char,
    lower_bound: usize,
    upper_bound: usize,
}

struct PasswordPolicy2 {
    char: char,
    lower_index: usize,
    upper_index: usize,
}

impl PasswordPolicyValidator for PasswordPolicy1 {
    fn is_policy_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.char).count();

        self.lower_bound <= count && count <= self.upper_bound
    }
}

impl PasswordPolicyValidator for PasswordPolicy2 {
    fn is_policy_valid(&self, password: &str) -> bool {
        let mut chunk = password
            .chars()
            .skip(self.lower_index)
            .take(self.upper_index - self.lower_index + 1);

        let c1 = chunk.next().unwrap();
        let c2 = chunk.last().unwrap();
        (c1 == self.char && c2 != self.char) || (c1 != self.char && c2 == self.char)
    }
}

impl<T> std::str::FromStr for PasswordEntry<T>
where
    T: PasswordPolicyValidator,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_policy, raw_password) = s.split(": ").collect_tuple().unwrap();
        let policy = raw_policy.parse::<T>().unwrap();

        Ok(PasswordEntry {
            password: raw_password.trim().to_string(),
            policy,
        })
    }
}

impl std::str::FromStr for PasswordPolicy1 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_bounds, raw_char) = s.trim().split(' ').collect_tuple().unwrap();
        let (raw_lower_bound, raw_upper_bound) =
            raw_bounds.trim().split('-').collect_tuple().unwrap();

        Ok(PasswordPolicy1 {
            char: raw_char.chars().next().unwrap(),
            lower_bound: raw_lower_bound.parse()?,
            upper_bound: raw_upper_bound.parse()?,
        })
    }
}

impl std::str::FromStr for PasswordPolicy2 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_bounds, raw_char) = s.trim().split(' ').collect_tuple().unwrap();
        let (raw_lower_index, raw_upper_index) =
            raw_bounds.trim().split('-').collect_tuple().unwrap();

        Ok(PasswordPolicy2 {
            char: raw_char.chars().next().unwrap(),
            lower_index: raw_lower_index.parse::<usize>()? - 1,
            upper_index: raw_upper_index.parse::<usize>()? - 1,
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
            .map(|l| l.parse::<PasswordEntry<PasswordPolicy1>>().unwrap())
            .collect();
        let solution = solve(input);

        assert_eq!(2, solution);
    }

    #[test]
    fn test_part2_sample_input() {
        let input = &mut vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|l| l.parse::<PasswordEntry<PasswordPolicy2>>().unwrap())
            .collect();
        let solution = solve(input);

        assert_eq!(1, solution);
    }

    #[test]
    fn test_part1_solution() {
        let mut input = read_input::<PasswordEntry<PasswordPolicy1>>(2020, 2).unwrap();
        let solution = solve(&mut input);
        assert_eq!(439, solution);
    }

    #[test]
    fn test_part2_solution() {
        let mut input = read_input::<PasswordEntry<PasswordPolicy2>>(2020, 2).unwrap();
        let solution = solve(&mut input);
        assert_eq!(584, solution);
    }
}
