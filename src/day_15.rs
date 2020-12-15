use std::collections::HashMap;

const DAY: u32 = 15;

type Input = Vec<u32>;
type Output = u32;

fn input_transformer(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect()
}

fn solve_part1(input: &Input) -> Output {
    solve(input, 2020)
}

fn solve_part2(input: &Input) -> Output {
    solve(input, 30_000_000)
}

fn solve(input: &Input, target: u32) -> Output {
    let start = (input.len() + 1) as u32;
    let mut seen = input
        .iter()
        .enumerate()
        .map(|(i, &e)| (e, (i as u32 + 1, i as u32 + 1)))
        .collect::<HashMap<_, _>>();

    let mut last_spoken = *input.last().unwrap();
    for i in start..target + 1 {
        let mut spoken = 0;
        if let Some((first, last)) = seen.get(&last_spoken) {
            if first != last {
                spoken = last - first;
            }
        }

        if let Some((first, last)) = seen.get(&spoken) {
            seen.insert(spoken, (*last, i));
        } else {
            seen.insert(spoken, (i, i));
        }

        last_spoken = spoken;
    }
    last_spoken
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(436, solve_part1(&input_transformer("0,3,6")));
        assert_eq!(1, solve_part1(&input_transformer("1,3,2")));
        assert_eq!(10, solve_part1(&input_transformer("2,1,3")));
        assert_eq!(27, solve_part1(&input_transformer("1,2,3")));
        assert_eq!(78, solve_part1(&input_transformer("2,3,1")));
        assert_eq!(438, solve_part1(&input_transformer("3,2,1")));
        assert_eq!(1836, solve_part1(&input_transformer("3,1,2")));
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(376, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(323780, solution);
    }
}
