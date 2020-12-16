use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DAY: u32 = 16;

#[derive(Debug, Clone)]
struct Prop {
    key: String,
    value: (usize, usize, usize, usize),
}

impl Prop {
    fn is_valid(&self, value: usize) -> bool {
        (self.value.0 <= value && value <= self.value.1)
            || (self.value.2 <= value && value <= self.value.3)
    }
}

#[derive(Debug, Clone)]
struct Notes {
    props: HashMap<String, Prop>,
    your_ticket: Vec<usize>,
    nearby_ticket: Vec<Vec<usize>>,
}

type Input = Notes;
type Output = usize;

fn input_transformer(input: &str) -> Input {
    let mut notes = Input {
        props: HashMap::new(),
        your_ticket: Vec::new(),
        nearby_ticket: Vec::new(),
    };

    let mut stage = 0;
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .for_each(|l| {
            if l == "your ticket:" || l == "nearby tickets:" {
                stage += 1;
            } else {
                match stage {
                    0 => {
                        let (key, value) = l.split(": ").collect_tuple().unwrap();
                        let value = value
                            .split(" or ")
                            .flat_map(|v| v.split('-'))
                            .map(|v| v.parse().unwrap())
                            .collect_tuple()
                            .unwrap();

                        notes.props.insert(
                            key.to_owned(),
                            Prop {
                                key: key.to_owned(),
                                value,
                            },
                        );
                    }
                    1 => {
                        notes.your_ticket = l.split(',').map(|x| x.parse().unwrap()).collect();
                    }
                    2 => {
                        notes
                            .nearby_ticket
                            .push(l.split(',').map(|x| x.parse().unwrap()).collect());
                    }
                    _ => panic!("Invalid stage!"),
                };
            }
        });

    notes
}

fn solve_part1(input: &Input) -> Output {
    input
        .nearby_ticket
        .iter()
        .flat_map(|t| {
            t.iter()
                .filter(|v| !input.props.values().any(|p| p.is_valid(**v)))
        })
        .sum()
}

fn solve_part2(input: &Input) -> Output {
    let valid_tickets: Vec<Vec<usize>> = input
        .nearby_ticket
        .iter()
        .filter(|t| {
            t.iter()
                .all(|v| input.props.values().any(|p| p.is_valid(*v)))
        })
        .cloned()
        .collect();

    let all_prop_keys: HashSet<String> = input.props.keys().cloned().collect();
    let mut is: HashSet<usize> = (0..input.your_ticket.len()).collect();
    let mut props_map = HashMap::<String, usize>::with_capacity(input.your_ticket.len());
    let mut i = 0;
    while input.your_ticket.len() != props_map.len() && i < input.your_ticket.len() {
        all_prop_keys
            .difference(&props_map.keys().cloned().collect())
            .map(|p| input.props.get(p).unwrap())
            .for_each(|p| {
                let possible = is
                    .iter()
                    .filter(|i| valid_tickets.iter().map(|t| t[**i]).all(|x| p.is_valid(x)))
                    .copied()
                    .exactly_one();

                if let Ok(index) = possible {
                    props_map.insert(p.key.to_owned(), index);
                    is.remove(&index);
                }
            });

        i += 1;
    }

    props_map
        .keys()
        .filter(|k| k.starts_with("departure"))
        .map(|k| input.your_ticket[*props_map.get(k).unwrap()])
        .product()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    #[test]
    fn test_part1_sample() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50
        
        your ticket:
        7,1,14
        
        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let solution = solve_part1(&input_transformer(input));

        assert_eq!(71, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(25916, solution);
    }

    #[test]
    fn test_part2_sample() {
        let input = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19
        
        your ticket:
        11,12,13
        
        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";
        let solution = solve_part2(&input_transformer(input));

        assert_eq!(1, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(2564529489989, solution);
    }
}
