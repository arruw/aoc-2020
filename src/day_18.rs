const DAY: u32 = 18;

type Input = Vec<String>;
type Output = u64;

peg::parser! {
    grammar math_parser() for str {
        rule number() -> u64
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule whitespace() = quiet!{[' ' | '\t']+}

        pub rule expression_part1() -> u64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            n:number() { n }
            "(" e:expression_part1() ")" { e }
        }

        pub rule expression_part2() -> u64 = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:number() { n }
            "(" e:expression_part2() ")" { e }
        }
    }
}

fn input_transformer(input: &str) -> Input {
    input.lines().map(|l| l.replace(' ', "")).collect()
}

fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|ex| math_parser::expression_part1(ex).ok())
        .sum()
}

fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|ex| math_parser::expression_part2(ex).ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    #[test]
    fn test_part1_sample() {
        let input = "2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let solution = solve_part1(&input_transformer(input));

        assert_eq!(26 + 437 + 12240 + 13632, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(12918250417632, solution);
    }

    #[test]
    fn test_part2_sample() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let solution = solve_part2(&input_transformer(input));

        assert_eq!(51 + 46 + 1445 + 669060 + 23340, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(171259538712010, solution);
    }
}
