const DAY: u32 = 11;

type Input = Vec<Vec<char>>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn solve_part1(input: Input) -> Output {
    iterate(input, 1, 4)
}

fn solve_part2(input: Input) -> Output {
    iterate(input, 0, 5)
}

fn iterate(input: Input, r: usize, l: usize) -> Output {
    let mut layout = input;

    let n = layout.len();
    let m = layout[0].len();

    loop {
        let mut changes = 0;
        let mut tmp = layout.clone();

        for i in 0..n {
            for j in 0..m {
                if layout[i][j] == '.' {
                    continue;
                }

                let occupied = count_occupied_seats(&layout, i, j, r);
                if layout[i][j] == 'L' && occupied == 0 {
                    tmp[i][j] = '#';
                    changes += 1;
                } else if layout[i][j] == '#' && occupied >= l {
                    tmp[i][j] = 'L';
                    changes += 1;
                }
            }
        }

        layout = tmp;

        if changes == 0 {
            break;
        }
    }

    layout
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#').count())
        .sum()
}

fn count_occupied_seats(layout: &Input, i: usize, j: usize, r: usize) -> usize {
    let n = layout.len() as isize;
    let m = layout[0].len() as isize;

    let mut occupied = 0;

    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    for (di, dj) in directions {
        let mut ii = i as isize + di;
        let mut jj = j as isize + dj;
        let mut d: usize = 1;
        while 0 <= ii && ii < n && 0 <= jj && jj < m {
            let x = layout[ii as usize][jj as usize];
            if x == '#' {
                occupied += 1;
                break;
            }
            if x == 'L' || d == r {
                break;
            }
            ii += di;
            jj += dj;
            d += 1;
        }
    }

    occupied
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(37, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(2481, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(26, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(2227, solution);
    }
}
