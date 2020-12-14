use itertools::Itertools;
use std::collections::HashMap;

const DAY: u32 = 14;

type Input = Vec<(Option<usize>, Vec<char>)>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            if l.starts_with("mask = ") {
                return (None, l[7..].chars().collect());
            }

            let (p1, p2) = l.split(" = ").collect_tuple().unwrap();
            let address = p1[4..p1.len() - 1].parse().unwrap();
            let value = format!("{:036b}", p2.parse::<usize>().unwrap())
                .chars()
                .collect();
            (Some(address), value)
        })
        .collect()
}

fn solve_part1(input: &Input) -> Output {
    let mut mask = vec![];
    let mut mem: HashMap<usize, Vec<char>> = HashMap::new();

    input.iter().for_each(|(address, value)| {
        if let Some(address) = address {
            let value: Vec<char> = mask
                .iter()
                .zip(value.iter())
                .map(|(m, b)| if *m == 'X' { *b } else { *m })
                .collect();
            mem.insert(*address, value);
        } else {
            mask = value.to_owned();
        }
    });

    mem.iter()
        .map(|(_, v)| {
            let v: String = v.iter().collect();
            usize::from_str_radix(&v, 2).unwrap()
        })
        .sum()
}

fn solve_part2(input: &Input) -> Output {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(&input_transformer(SAMPLE));

        assert_eq!(165, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(17481577045893, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(&input_transformer(SAMPLE));

        assert_eq!(0, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(0, solution);
    }
}
