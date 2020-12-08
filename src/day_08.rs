use crate::utils::{modulo, parse_input, Error};
use std::collections::*;
use std::str::FromStr;

const DAY: u32 = 8;

type Input = Vec<Instruction>;
type Output = isize;

enum Op {
    Nop,
    Acc,
    Jmp,
}

struct Instruction(Op, isize);

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            op => panic!("Unknown operation '{}'!", op),
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s[..3].parse()?, s[4..].parse()?))
    }
}

impl Instruction {
    fn execute(&self, reg: isize, index: usize) -> (isize, isize) {
        match self {
            Instruction(Op::Nop, _) => (reg, (index + 1) as isize),
            Instruction(Op::Acc, arg) => (reg + arg, (index + 1) as isize),
            Instruction(Op::Jmp, arg) => (reg, (index as isize) + arg),
        }
    }

    fn flip(&mut self) {
        match self.0 {
            Op::Jmp => self.0 = Op::Nop,
            Op::Nop => self.0 = Op::Jmp,
            Op::Acc => (),
        };
    }
}

fn boot(instructions: &[Instruction]) -> (bool, isize, HashSet<usize>) {
    let len = instructions.len();
    let mut index = 0;
    let mut reg = 0;
    let mut visited = hashset![];
    while !visited.contains(&index) {
        visited.insert(index);

        let instruction = &instructions[index];
        let (new_reg, new_index) = instruction.execute(reg, index);

        if index == len - 1 {
            return (true, new_reg, visited);
        }

        reg = new_reg;
        index = modulo(new_index, len);
    }
    (false, reg, visited)
}

fn input_transformer(input: &str) -> Input {
    parse_input(input)
}

fn solve_part1(input: &Input) -> isize {
    let (_, reg, _) = boot(input);
    reg
}

fn solve_part2(input: &mut Input) -> Output {
    let (_, _, visited) = boot(input);
    for index in visited {
        if let Op::Acc = input[index].0 {
            continue;
        }
        input[index].flip();
        let (booted, reg, _) = boot(input);
        input[index].flip();
        if booted {
            return reg;
        }
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
        let solution = solve_part1(&input_transformer(SAMPLE));

        assert_eq!(5, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(1709, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(&mut input_transformer(SAMPLE));

        assert_eq!(8, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&mut input_transformer(&input));

        assert_eq!(1976, solution);
    }
}
