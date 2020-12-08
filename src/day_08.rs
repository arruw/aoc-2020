use itertools::Itertools;

const DAY: u32 = 8;

type Input = Vec<(String, isize)>;
type Output = isize;

// struct Nop()
// struct Acc(isize)
// struct Jmp(isize)

// trait Exec {
//     fn exec(&self, arg: isize)
// }

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.split(' ')
                .map(|p| p.trim().to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .map(|(op, arg)| (op, arg.parse::<isize>().unwrap()))
        .collect()
}

fn solve_part1(input: &Input) -> (isize, Vec<bool>) {
    let len = input.len();
    let mut visited = vec![false; len];
    let mut acc = 0;
    let mut i = 0;
    while !visited[i] {
        visited[i] = true;
        let (op, arg) = &input[i];
        match &op[..] {
            "acc" => {
                acc += arg;
                i += 1;
            }
            "jmp" => i = ((i as isize) + arg % (len as isize)) as usize,
            _ => i += 1,
        }
    }
    (acc, visited)
}

fn will_terminate(input: &Input, change_op_at: usize) -> (bool, isize) {
    let len = input.len();
    let mut visited = vec![false; len];
    let mut acc = 0;
    let mut i = 0;
    while !visited[i] {
        visited[i] = true;
        let (op, arg) = &input[i];
        let mut op = op.to_owned();
        if i == change_op_at {
            match &op[..] {
                "nop" => op = "jmp".to_string(),
                "jmp" => op = "nop".to_string(),
                _ => (),
            }
        }
        let mut next_i = i;
        match &op[..] {
            "acc" => {
                acc += arg;
                next_i += 1;
            }
            "jmp" => next_i = ((next_i as isize) + arg % (len as isize)) as usize,
            "nop" => next_i += 1,
            ins => panic!("Unknown instruction '{}'!", ins),
        }
        if i == len - 1 {
            return (true, acc);
        }
        i = next_i;
    }
    (false, acc)
}

fn solve_part2(input: Input) -> Output {
    let len = input.len();
    let (_, visited) = solve_part1(&input);
    for i in 0..len {
        if !visited[i] {
            continue;
        }

        let (ended, acc) = will_terminate(&input, i);
        if !ended {
            continue;
        }

        return acc;
    }
    panic!("No solution!");
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";

    #[test]
    fn test_part1_sample() {
        let (solution, _) = solve_part1(&input_transformer(SAMPLE));

        assert_eq!(5, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let (solution, _) = solve_part1(&input_transformer(&input));

        assert_eq!(1709, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(8, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(1976, solution);
    }
}
