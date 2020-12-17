use std::collections::HashSet;

use itertools::Itertools;

const DAY: u32 = 17;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Coordinate(isize, isize, isize, isize);

impl Coordinate {
    const NEIGHBOR_DELTAS: [(isize, isize, isize, isize); 80] = [
        (-1, -1, -1, -1),
        (-1, -1, -1, 0),
        (-1, -1, -1, 1),
        (-1, -1, 0, -1),
        (-1, -1, 0, 0),
        (-1, -1, 0, 1),
        (-1, -1, 1, -1),
        (-1, -1, 1, 0),
        (-1, -1, 1, 1),
        (-1, 0, -1, -1),
        (-1, 0, -1, 0),
        (-1, 0, -1, 1),
        (-1, 0, 0, -1),
        (-1, 0, 0, 0),
        (-1, 0, 0, 1),
        (-1, 0, 1, -1),
        (-1, 0, 1, 0),
        (-1, 0, 1, 1),
        (-1, 1, -1, -1),
        (-1, 1, -1, 0),
        (-1, 1, -1, 1),
        (-1, 1, 0, -1),
        (-1, 1, 0, 0),
        (-1, 1, 0, 1),
        (-1, 1, 1, -1),
        (-1, 1, 1, 0),
        (-1, 1, 1, 1),
        (0, -1, -1, -1),
        (0, -1, -1, 0),
        (0, -1, -1, 1),
        (0, -1, 0, -1),
        (0, -1, 0, 0),
        (0, -1, 0, 1),
        (0, -1, 1, -1),
        (0, -1, 1, 0),
        (0, -1, 1, 1),
        (0, 0, -1, -1),
        (0, 0, -1, 0),
        (0, 0, -1, 1),
        (0, 0, 0, -1),
        (0, 0, 0, 1),
        (0, 0, 1, -1),
        (0, 0, 1, 0),
        (0, 0, 1, 1),
        (0, 1, -1, -1),
        (0, 1, -1, 0),
        (0, 1, -1, 1),
        (0, 1, 0, -1),
        (0, 1, 0, 0),
        (0, 1, 0, 1),
        (0, 1, 1, -1),
        (0, 1, 1, 0),
        (0, 1, 1, 1),
        (1, -1, -1, -1),
        (1, -1, -1, 0),
        (1, -1, -1, 1),
        (1, -1, 0, -1),
        (1, -1, 0, 0),
        (1, -1, 0, 1),
        (1, -1, 1, -1),
        (1, -1, 1, 0),
        (1, -1, 1, 1),
        (1, 0, -1, -1),
        (1, 0, -1, 0),
        (1, 0, -1, 1),
        (1, 0, 0, -1),
        (1, 0, 0, 0),
        (1, 0, 0, 1),
        (1, 0, 1, -1),
        (1, 0, 1, 0),
        (1, 0, 1, 1),
        (1, 1, -1, -1),
        (1, 1, -1, 0),
        (1, 1, -1, 1),
        (1, 1, 0, -1),
        (1, 1, 0, 0),
        (1, 1, 0, 1),
        (1, 1, 1, -1),
        (1, 1, 1, 0),
        (1, 1, 1, 1),
    ];
    fn neighbors(&self) -> impl Iterator<Item = Coordinate> + '_ {
        Coordinate::NEIGHBOR_DELTAS
            .iter()
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
    let mut active: HashSet<Coordinate> = input;

    (0..6).for_each(|i| {
        let snapshot = active.to_owned();

        // Rule #1
        snapshot
            .iter()
            .filter(|c| {
                let count = c
                    .neighbors()
                    .filter(|c| c.3 == 0)
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
            .flat_map(|c| c.neighbors().filter(|c| c.3 == 0))
            .filter(|c| !snapshot.contains(c))
            .filter(|c| {
                let count = c
                    .neighbors()
                    .filter(|c| c.3 == 0)
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

fn solve_part2(input: Input) -> Output {
    let mut active: HashSet<Coordinate> = input;

    (0..6).for_each(|i| {
        let snapshot = active.to_owned();

        // Rule #1
        snapshot
            .iter()
            .filter(|c| {
                let count = c
                    .neighbors()
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
            .flat_map(|c| c.neighbors())
            .filter(|c| !snapshot.contains(c))
            .filter(|c| {
                let count = c
                    .neighbors()
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
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(848, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(2280, solution);
    }
}
