use crate::utils::*;
use itertools::Itertools;
use std::str::FromStr;

const DAY: u32 = 5;

struct BordingPass(String);

impl BordingPass {
    fn row(&self) -> usize {
        let binary = &self.0[..7].replace("F", "0").replace("B", "1");
        usize::from_str_radix(binary, 2).unwrap()
    }

    fn column(&self) -> usize {
        let binary = &self.0[7..].replace("L", "0").replace("R", "1");
        usize::from_str_radix(binary, 2).unwrap()
    }

    pub fn seat_id(&self) -> usize {
        self.row() * 8 + self.column()
    }
}

impl FromStr for BordingPass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(BordingPass(s.to_string()))
    }
}

type Input = Vec<BordingPass>;
type Output = Option<usize>;

fn input_transformer(input: &str) -> Input {
    parse_input(input)
}

fn solve_part1(input: Input) -> Output {
    input.iter().map(|l| l.seat_id()).max()
}

fn solve_part2(input: Input) -> Output {
    let seats: Vec<usize> = input.iter().map(|l| l.seat_id()).sorted().collect();

    for i in 1..seats.len() {
        let prev = seats[i - 1];
        let curr = seats[i];
        if prev + 2 == curr {
            return Some(prev + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, BordingPass, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL";

    #[test]
    fn test_bordingpass() {
        let bp1: BordingPass = "BFFFBBFRRR".parse().unwrap();
        let bp2: BordingPass = "FFFBBBFRRR".parse().unwrap();
        let bp3: BordingPass = "BBFFBBFRLL".parse().unwrap();

        assert_eq!(70, bp1.row());
        assert_eq!(7, bp1.column());
        assert_eq!(567, bp1.seat_id());

        assert_eq!(14, bp2.row());
        assert_eq!(7, bp2.column());
        assert_eq!(119, bp2.seat_id());

        assert_eq!(102, bp3.row());
        assert_eq!(4, bp3.column());
        assert_eq!(820, bp3.seat_id());
    }

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(Some(820), solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(Some(974), solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(None, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(Some(646), solution);
    }
}
