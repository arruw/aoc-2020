use itertools::Itertools;
use std::collections::HashMap;

const DAY: u32 = 14;

type Input = Vec<(Option<usize>, String)>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            if let Some(l) = l.strip_prefix("mask = ") {
                return (None, l.to_string());
            }

            let (p1, p2) = l.split(" = ").collect_tuple().unwrap();
            let address = p1[4..p1.len() - 1].parse().unwrap();
            let value = format!("{:036b}", p2.parse::<usize>().unwrap());
            (Some(address), value)
        })
        .collect()
}

fn solve_part1(input: &Input) -> Output {
    let mut mask: String = "".to_string();
    let mut mem: HashMap<usize, Vec<char>> = HashMap::new();

    input.iter().for_each(|(address, value)| {
        if let Some(address) = address {
            let value: Vec<char> = mask
                .chars()
                .zip(value.chars())
                .map(|(m, b)| if m == 'X' { b } else { m })
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
    let mut mask: String = "".to_string();
    let mut mem: HashMap<usize, usize> = HashMap::new();

    input.iter().for_each(|(address, value)| {
        if let Some(address) = address {
            let value = usize::from_str_radix(value, 2).unwrap();
            let address = format!("{:036b}", address);
            let floating_address: Vec<char> = mask
                .chars()
                .zip(address.chars())
                .map(|(m, a)| if m == '0' { a } else { m })
                .collect();

            to_addresses(floating_address).for_each(|address| {
                mem.insert(address, value);
            });
            println!();
        } else {
            mask = value.to_owned();
        }
    });

    mem.values().sum()
}

fn to_addresses(floating_address: Vec<char>) -> impl Iterator<Item = usize> {
    let x_count = floating_address.iter().filter(|c| **c == 'X').count();

    (0..2usize.pow(x_count as u32))
        .zip(std::iter::repeat(x_count))
        .map(|(x, x_count)| {
            format!("{:036b}", x)
                .chars()
                .skip(36 - x_count)
                .collect::<Vec<char>>()
        })
        .zip(std::iter::repeat(floating_address))
        .map(|(f_bits, f_address)| {
            let mut i: usize = 0;
            let mut address: Vec<char> = vec![];
            for f_address_bit in f_address {
                if f_address_bit == 'X' {
                    address.push(f_bits[i]);
                    i += 1;
                } else {
                    address.push(f_address_bit);
                }
            }
            address
        })
        .map(|address| {
            let address: String = address.iter().collect();
            usize::from_str_radix(&address, 2).unwrap()
        })
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    #[test]
    fn test_part1_sample() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
        let solution = solve_part1(&input_transformer(input));

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
        let input = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
        let solution = solve_part2(&input_transformer(input));

        assert_eq!(208, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(4160009892257, solution);
    }
}
