use std::collections::*;

const DAY: u32 = 6;

type Input = Vec<Vec<HashSet<char>>>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    let mut groups: Vec<Vec<HashSet<char>>> = vec![];
    let mut group: Vec<HashSet<char>> = vec![];
    for l in input.lines().map(|l| l.trim()) {
        if l.is_empty() && !group.is_empty() {
            groups.push(group.clone());
            group.clear();
        } else if !l.is_empty() {
            group.push(l.chars().collect())
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }

    groups
}

fn solve_part1(input: Input) -> Output {
    input
        .iter()
        .map(|g| {
            g.iter().fold(None, |acc, p| -> Option<HashSet<char>> {
                let p = p.iter().copied().collect();
                acc.map(|a| a.union(&p).copied().collect()).or(Some(p))
            })
        })
        .map(|g| g.unwrap().len())
        .sum()
}

fn solve_part2(input: Input) -> Output {
    input
        .iter()
        .map(|g| {
            g.iter().fold(None, |acc, p| -> Option<HashSet<char>> {
                let p = p.iter().copied().collect();
                acc.map(|a| a.intersection(&p).copied().collect())
                    .or(Some(p))
            })
        })
        .map(|g| g.unwrap().len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(11, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(6778, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(6, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(3406, solution);
    }
}
