use std::collections::{BTreeMap, HashMap};

use crate::utils::*;

const DAY: u32 = 10;

fn input_transformer(input: &str) -> Vec<i32> {
    parse_input(input)
}

fn solve_part1(input: &[i32]) -> usize {
    let mut input = input.to_vec();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);

    let mut counter = HashMap::<i32, usize>::new();

    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];

        *counter.entry(diff).or_insert(0) += 1;
    }

    counter.get(&1).unwrap_or(&0) * counter.get(&3).unwrap_or(&0)
}

fn solve_part2(input: &[i32]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let last = input[input.len() - 1] + 3;
    input.push(last);

    let mut counter = BTreeMap::<i32, usize>::new();
    counter.insert(0, 1);

    for adapter in input {
        let prevs: Vec<(i32, usize)> = counter
            .iter()
            .filter(|(k, v)| adapter - **k >= 1)
            .filter(|(k, v)| adapter - **k <= 3)
            .map(|(k, v)| (*k, *v))
            .collect();
        for (k, v) in prevs {
            *counter.entry(adapter).or_insert(0) += v;
        }
    }

    *counter.get(&last).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE1: &str = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    const SAMPLE2: &str = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    #[test]
    fn test_part1_sample1() {
        let solution = solve_part1(&input_transformer(SAMPLE1));

        assert_eq!(7 * 5, solution);
    }

    #[test]
    fn test_part1_sample2() {
        let solution = solve_part1(&input_transformer(SAMPLE2));

        assert_eq!(22 * 10, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(2263, solution);
    }

    #[test]
    fn test_part2_sample1() {
        let solution = solve_part2(&input_transformer(SAMPLE1));

        assert_eq!(8, solution);
    }

    #[test]
    fn test_part2_sample2() {
        let solution = solve_part2(&input_transformer(SAMPLE2));

        assert_eq!(19208, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(396857386627072, solution);
    }
}
