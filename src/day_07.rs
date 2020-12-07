use itertools::Itertools;
use std::collections::*;

const DAY: u32 = 7;

type Input = HashMap<String, HashMap<String, usize>>;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.replace("bags", "")
                .replace("bag", "")
                .replace(".", "")
                .trim()
                .to_string()
        })
        .map(|l| {
            let (color, contains) = l
                .split("contain")
                .map(|x| x.trim())
                .collect_tuple()
                .unwrap();
            let contains: HashMap<String, usize> = contains
                .split(',')
                .filter(|x| *x != "no other")
                .map(|x| {
                    let (count, color) = x.trim().split_once(' ').unwrap();
                    let count: usize = count.parse().unwrap();
                    (color.to_string(), count)
                })
                .collect();

            (color.to_owned(), contains)
        })
        .collect()
}

fn solve_part1(input: Input) -> Output {
    let mut solution = hashset![];

    let mut already_searched = hashset![];
    let mut search_colors = hashset!["shiny gold".to_string()];
    while !search_colors.is_empty() {
        let mut next_search_colors = hashset![];
        for (color, contains) in input.iter() {
            let contains_keys: HashSet<String> = contains.keys().cloned().collect();
            if search_colors.intersection(&contains_keys).count() > 0 {
                solution.insert(color.to_owned());
                if !already_searched.contains(color) {
                    next_search_colors.insert(color.to_owned());
                }
            }
        }
        already_searched = already_searched.union(&search_colors).cloned().collect();
        search_colors = next_search_colors;
    }

    solution.len()
}

fn solve_part2(input: Input) -> Output {
    let mut searched: HashMap<String, usize> = input
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, v)| (k.to_string(), 0))
        .collect();

    let input_keys: HashSet<String> = input.keys().cloned().collect();
    loop {
        let searched_keys: HashSet<String> = searched.keys().cloned().collect();
        for color in input_keys.difference(&searched_keys) {
            let color_keys: HashSet<String> = input.get(color).unwrap().keys().cloned().collect();
            if color_keys.difference(&searched_keys).count() > 0 {
                continue;
            }

            print!("{:?} = ", color);
            let n = input.get(color).unwrap().iter().fold(0, |acc, (k, v)| {
                let x = searched.get(k).unwrap();
                print!("({} + {} * {}) + ", v, v, x);
                acc + v + v * x
            });
            println!("0 = {:?}", n);

            searched.insert(color.to_owned(), n);

            if color == "shiny gold" {
                return searched.get(color).unwrap().to_owned();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(4, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(192, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(input_transformer(SAMPLE));

        assert_eq!(32, solution);
    }

    #[test]
    fn test_part2_sample_2() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";
        let solution = solve_part2(input_transformer(input));

        assert_eq!(126, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        assert_eq!(12128, solution);
    }
}
