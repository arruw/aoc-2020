use itertools::Itertools;

use crate::utils::*;

const DAY: u32 = 9;

fn input_transformer(input: &str) -> Vec<i64> {
    parse_input(input)
}

// O(n^2 log n)
fn solve_part1(input: &[i64], preamble_size: usize) -> Option<i64> {
    let mut preamble: Vec<i64> = input[..preamble_size].to_vec();
    preamble.sort_unstable();

    for i in preamble_size..input.len() {
        if !is_ok(&preamble, input[i]) {
            return Some(input[i]);
        }

        preamble.swap_remove(preamble.binary_search(&input[i - preamble_size]).unwrap());
        preamble.push(input[i]);
        preamble.sort_unstable();
    }
    None
}

// O(n)
fn solve_part2(input: &[i64], target_sum: i64) -> Option<i64> {
    let mut l = 0;
    let mut r = 1;
    let mut sum = input[l] + input[r];
    while l < input.len() && r < input.len() {
        match sum {
            x if x < target_sum => {
                r += 1;
                sum += input[r];
            }
            x if x > target_sum => {
                sum -= input[l];
                l += 1;
            }
            _ => {
                let (min, max) = input[l..r + 1].iter().minmax().into_option().unwrap();
                return Some(min + max);
            }
        }
    }
    None
}

fn is_ok(input: &[i64], target_sum: i64) -> bool {
    let mut l = 0;
    let mut r = input.len() - 1;
    while l < r {
        match input[l] + input[r] {
            sum if sum == target_sum => return true,
            sum if sum < target_sum => l += 1,
            _ => r -= 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(&input_transformer(SAMPLE), 5);

        assert_eq!(Some(127), solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input), 25);

        assert_eq!(Some(1398413738), solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(&input_transformer(SAMPLE), 127);

        assert_eq!(Some(62), solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input), 1398413738);

        assert_eq!(Some(169521051), solution);
    }
}
