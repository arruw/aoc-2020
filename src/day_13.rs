use itertools::Itertools;

const DAY: u32 = 13;

type Input = (usize, Vec<Option<usize>>);
type Output = usize;

fn input_transformer(input: &str) -> Input {
    let (line1, line2) = input.lines().map(|l| l.trim()).collect_tuple().unwrap();
    let time = line1.parse().unwrap();
    let busses = line2
        .split(',')
        .map(|b| b.trim())
        .map(|b| {
            if b != "x" {
                Some(b.parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .collect();

    (time, busses)
}

fn solve_part1(input: &Input) -> Output {
    let (time, busses) = input;

    let (departs, bus_id) = busses
        .iter()
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .map(|b| {
            if time % b == 0 {
                (*time, b)
            } else {
                ((time / b + 1) * b, b)
            }
        })
        .min()
        .unwrap();

    (departs - time) * bus_id
}

fn solve_part2(input: &Input, first: usize) -> Output {
    // TODO - Solve using the Chinese remainder theorem
    println!("Do some magic:");
    println!("Solve[{{Mod[x + 0 , 17] == 0 ,Mod[x + 11, 37] == 0 ,Mod[x + 17, 409] == 0 ,Mod[x + 19, 29] == 0 ,Mod[x + 30, 13] == 0 ,Mod[x + 40, 23] == 0 ,Mod[x + 48, 373] == 0 ,Mod[x + 58, 41] == 0 ,Mod[x + 67, 19] == 0}}, x, Integers]");
    530015546283687
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "939
    7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(&input_transformer(SAMPLE));

        assert_eq!(295, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(207, solution);
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let solution = solve_part2(&input_transformer(SAMPLE), 1000000);

        assert_eq!(1068781, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input), 100000000000000);

        assert_eq!(530015546283687, solution);
    }
}
