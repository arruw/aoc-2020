use pest::Parser;

const DAY: u32 = 19;

type Input = Vec<String>;
type Output = usize;

mod puzzle_1_sample {
    #[derive(Parser)]
    #[grammar = "day_19/day_19_puzzle1_sample.pest"]
    pub struct Parser;
}

mod puzzle_1 {
    #[derive(Parser)]
    #[grammar = "day_19/day_19_puzzle1.pest"]
    pub struct Parser;
}

mod puzzle_2_sample {
    #[derive(Parser)]
    #[grammar = "day_19/day_19_puzzle2_sample.pest"]
    pub struct Parser;
}

mod puzzle_2 {
    #[derive(Parser)]
    #[grammar = "day_19/day_19_puzzle2.pest"]
    pub struct Parser;
}

fn input_transformer(input: &str) -> Input {
    let init = (true, &mut Vec::<String>::new());
    let expressions = input
        .lines()
        .map(|l| l.trim())
        .fold(init, |(skip, expressions), l| {
            if skip && l.is_empty() {
                return (false, expressions);
            } else if !skip {
                expressions.push(l.to_owned())
            }

            (skip, expressions)
        })
        .1;

    expressions.to_owned()
}

fn solve_part1_sample(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|l| puzzle_1_sample::Parser::parse(puzzle_1_sample::Rule::R0, l).ok())
        .count()
}

fn solve_part1_puzzle(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|l| puzzle_1::Parser::parse(puzzle_1::Rule::R0, l).ok())
        .count()
}

fn solve_part2_sample(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|l| {
            if puzzle_2_sample::Parser::parse(puzzle_2_sample::Rule::R0, l).is_ok() {
                println!("OK: {}", l);
                Some(())
            } else {
                println!("NO: {}", l);
                None
            }
        })
        .count()
}

fn solve_part2_puzzle(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|l| puzzle_2::Parser::parse(puzzle_2::Rule::R0, l).ok())
        .count()
}

#[cfg(test)]
mod tests {
    use super::{
        input_transformer, solve_part1_puzzle, solve_part1_sample, solve_part2_puzzle,
        solve_part2_sample, DAY,
    };
    use crate::utils::*;

    #[test]
    fn test_part1_sample() {
        let input = "0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb";
        let solution = solve_part1_sample(&input_transformer(input));

        assert_eq!(2, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1_puzzle(&input_transformer(&input));

        assert_eq!(203, solution);
    }

    #[test]
    #[ignore = "not working"]
    fn test_part2_sample() {
        let input = "42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1
        
        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let solution = solve_part2_sample(&input_transformer(input));

        assert_eq!(12, solution);
    }

    #[test]
    #[ignore = "not working & unknown solution"]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2_puzzle(&input_transformer(&input));

        assert_eq!(0, solution);
    }
}
