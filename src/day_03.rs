type Input = Vec<Vec<char>>;
type Output = u64;

fn input_transformer(input: &str) -> Input {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn solve_part1(input: Input) -> Output {
    solve_slope(&input, &(3, 1))
}

fn solve_part2(input: Input) -> Output {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().map(|s| solve_slope(&input, &s)).product()
}

type Slope = (usize, usize);
fn solve_slope(input: &[Vec<char>], (dc, dr): &Slope) -> Output {
    let clen = input[0].len();
    let rlen = input.len();
    let mut count: u64 = 0;
    let mut r = 0;
    let mut c = 0;
    while r < rlen {
        if input[r][c] == '#' {
            count += 1;
        }
        r += dr;
        c = (c + dc) % clen;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2};
    use crate::utils::*;

    const SAMPLE: &str = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(7, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, 3).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(272, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(336, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, 3).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(3898725600, solution);
    }
}
