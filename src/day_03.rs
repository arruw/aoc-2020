type Input = Vec<Vec<char>>;
type Output = u64;

#[allow(dead_code)]
fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[allow(dead_code)]
fn solve_part1(input: Input) -> Output {
    solve_slope(&input, &(3, 1))
}

#[allow(dead_code)]
fn solve_part2(input: Input) -> Output {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().map(|s| solve_slope(&input, &s)).product()
}

type Slope = (usize, usize);
fn solve_slope(input: &Input, (dc, dr): &Slope) -> Output {
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
    use super::{input_generator, solve_part1, solve_part2};
    use crate::read_input;

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
    fn test_sample_part1() {
        let solution = solve_part1(input_generator(SAMPLE));

        assert_eq!(7, solution);
    }

    #[test]
    fn test_sample_part2() {
        let solution = solve_part2(input_generator(SAMPLE));

        assert_eq!(336, solution);
    }

    #[test]
    fn test_input_part1() {
        let input = read_input(2020, 3).unwrap();
        let solution = solve_part1(input_generator(&input));

        assert_eq!(272, solution);
    }

    #[test]
    fn test_input_part2() {
        let input = read_input(2020, 3).unwrap();
        let solution = solve_part2(input_generator(&input));

        assert_eq!(3898725600, solution);
    }
}
