use crate::utils::*;

type Input = Vec<i64>;
type Output = i64;

fn input_transformer(input: &str) -> Input {
    let mut input: Vec<i64> = parse_input(input);
    input.sort_unstable();
    input
}

fn solve_part1(input: Input) -> Output {
    let (a, b) = sum2n(&input, 2020).unwrap();
    input[a] * input[b]
}

fn solve_part2(input: Input) -> Output {
    let (a, b, c) = sum3n(&input, 2020).unwrap();
    input[a] * input[b] * input[c]
}

/// Find two numbers that add to the `target_sum`
///
/// Assumes sorted `input`
///
/// Time:   O(n)
/// Space:  O(1)
fn sum2n(input: &[i64], target_sum: i64) -> Option<(usize, usize)> {
    let mut l = 0;
    let mut r = input.len() - 1;
    while l < r {
        match input[l] + input[r] {
            sum if sum == target_sum => return Some((l, r)),
            sum if sum < target_sum => l += 1,
            _ => r -= 1,
        }
    }

    None
}

/// Find three numbers that add to the `target_sum`
///
/// Assumes sorted `input`
///
/// Time:   O(n^2)
/// Space:  O(1)
fn sum3n(input: &[i64], target_sum: i64) -> Option<(usize, usize, usize)> {
    for k in 0..input.len() {
        match sum2n(input, target_sum - input[k]) {
            Some((i, j)) => return Some((i, j, k)),
            _ => continue,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2};
    use crate::utils::*;

    const DAY: u32 = 1;
    const SAMPLE: &str = "1721
        979
        366
        299
        675
        1456";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(514579, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(658899, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(241861950, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(155806250, solution);
    }
}
