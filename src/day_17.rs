use std::collections::HashSet;

use itertools::Itertools;

const DAY: u32 = 17;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Coordinate(isize, isize, isize, isize);

enum Dimension {
    Three,
    Four,
}

lazy_static! {
    static ref NEIGHBOR_DELTAS: Vec<(isize, isize, isize, isize)> =
        [-1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1]
            .iter()
            .permutations(4)
            .unique()
            .map(|p| p
                .into_iter()
                .copied()
                .collect_tuple::<(isize, isize, isize, isize)>()
                .unwrap())
            .filter(|p| *p != (0, 0, 0, 0))
            .collect();
}

impl Coordinate {
    fn neighbors<'a>(&'a self, dim: &'a Dimension) -> impl Iterator<Item = Coordinate> + '_ {
        NEIGHBOR_DELTAS
            .iter()
            .filter(move |d| match dim {
                Dimension::Three => d.3 == 0,
                _ => true,
            })
            .map(move |(dx, dy, dz, dw)| {
                Coordinate(self.0 + dx, self.1 + dy, self.2 + dz, self.3 + dw)
            })
    }
}

type Input = HashSet<Coordinate>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, _)| Coordinate(x as isize, y as isize, 0isize, 0isize))
        })
        .collect()
}

fn solve_part1(input: Input) -> Output {
    solve_space(input, Dimension::Three, 6)
}

fn solve_part2(input: Input) -> Output {
    solve_space(input, Dimension::Four, 6)
}

fn solve_space(input: Input, dim: Dimension, cycles: u32) -> Output {
    let mut active: HashSet<Coordinate> = input;

    (0..cycles).for_each(|i| {
        let snapshot = active.to_owned();

        // Rule #1
        snapshot
            .iter()
            .filter(|c| {
                let count = c
                    .neighbors(&dim)
                    .collect::<HashSet<_>>()
                    .intersection(&snapshot)
                    .count();

                !(2 == count || 3 == count)
            })
            .for_each(|c| {
                active.remove(c);
            });

        // Rule #2
        snapshot
            .iter()
            .flat_map(|c| c.neighbors(&dim))
            .filter(|c| !snapshot.contains(c))
            .filter(|c| {
                let count = c
                    .neighbors(&dim)
                    .collect::<HashSet<_>>()
                    .intersection(&snapshot)
                    .count();

                count == 3
            })
            .unique()
            .for_each(|c| {
                active.insert(c);
            });
    });

    active.len()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = ".#.
    ..#
    ###";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(112, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(388, solution);
    }

    #[test]
    #[ignore = "too long"]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(848, solution);
    }

    #[test]
    #[ignore = "too long"]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(2280, solution);
    }
}
