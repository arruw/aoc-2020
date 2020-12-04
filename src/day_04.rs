use crate::utils::*;

use std::{collections::hash_map::RandomState, iter::FromIterator};
use std::{collections::*, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 4;

type Input<'a> = Vec<HashMap<String, String>>;
type Output = usize;

lazy_static! {
    static ref KV_RE: Regex = Regex::new(r"\b(?P<key>\w{3}):(?P<value>#?\w+)\b").unwrap();
}

lazy_static! {
    static ref VALUE_RE: HashMap<String, Regex> = hashmap! {
        "byr".to_string() => Regex::new(r"(\d{4})\b").unwrap(),
        "iyr".to_string() => Regex::new(r"(\d{4})\b").unwrap(),
        "eyr".to_string() => Regex::new(r"(\d{4})\b").unwrap(),
        "hgt".to_string() => Regex::new(r"(\d+)(in|cm)\b").unwrap(),
        "hcl".to_string() => Regex::new(r"(#\w{6})\b").unwrap(),
        "ecl".to_string() => Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap(),
        "pid".to_string() => Regex::new(r"(\d{9})\b").unwrap(),
    };
}

// lazy_static! {
//     static ref VALUE_RE: HashMap<&str, Regex> = hashmap! {
//         "byr" => Regex::new(r"(\d{4})\b").unwrap(),
//         "iyr" => Regex::new(r"(\d{4})\b").unwrap(),
//         "eyr" => Regex::new(r"(\d{4})\b").unwrap(),
//         "hgt" => Regex::new(r"(\d+)(in|cm)\b").unwrap(),
//         "hcl" => Regex::new(r"(#\w{6})\b").unwrap(),
//         "ecl" => Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap(),
//         "pid" => Regex::new(r"(\d{9})\b").unwrap(),
//     };
// }

fn input_transformer(input: &str) -> Input {
    let mut raw_passports: Vec<String> = vec![];
    let mut buffer = String::new();
    for l in input.lines() {
        if l.trim().is_empty() {
            raw_passports.push(buffer.trim().to_string());
            buffer.clear();
        } else {
            buffer.push_str(l.trim());
        }
    }
    if !buffer.is_empty() {
        raw_passports.push(buffer.trim().to_string());
        buffer.clear();
    }

    let mut passports: Vec<HashMap<String, String>> = vec![];
    for p in raw_passports {
        let passport = KV_RE
            .captures_iter(&p)
            .map(|c| {
                (
                    c.name("key").unwrap().as_str().to_string(),
                    c.name("value").unwrap().as_str().to_string(),
                )
            })
            .collect();

        passports.push(passport);
    }

    passports
}

fn solve_part1(input: Input) -> Output {
    let x: Vec<_> = VALUE_RE.keys().into_iter().collect();
    let required: HashSet<_> = VALUE_RE.keys().into_iter().collect();
    let count = input
        .iter()
        .filter(|p| {
            let found: HashSet<&String> = p.keys().into_iter().collect();

            let diff = required.difference(&found);

            let diff = diff.count() == 0;

            diff
        })
        .count();

    count
}

fn solve_part2(input: Input) -> Output {
    // let kv = Regex::new(r"\b(?P<key>byr|iyr|eyr|hgt|hcl|ecl|pid):(?P<value>#?\w+)\b").unwrap();
    // let rules = hashmap! {
    //     "byr" => Regex::new(r"(\d{4})\b").unwrap(),
    //     "iyr" => Regex::new(r"(\d{4})\b").unwrap(),
    //     "eyr" => Regex::new(r"(\d{4})\b").unwrap(),
    //     "hgt" => Regex::new(r"(\d+)(in|cm)\b").unwrap(),
    //     "hcl" => Regex::new(r"(#\w{6})\b").unwrap(),
    //     "ecl" => Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap(),
    //     "pid" => Regex::new(r"(\d{9})\b").unwrap(),
    // };

    // input
    //     .iter()
    //     .filter(|l| {
    //         let valid_keys: Vec<&str> = kv
    //             .captures_iter(l)
    //             .map(|c| {
    //                 (
    //                     c.name("key").unwrap().as_str(),
    //                     c.name("value").unwrap().as_str(),
    //                 )
    //             })
    //             .filter(|(k, _)| rules.contains_key(k))
    //             .filter(|(k, v)| {
    //                 let ok = rules.get(k).unwrap().is_match(v);
    //                 let ok = ok
    //                     && match &k[..] {
    //                         "byr" => str_num_between(v, 1920, 2002),
    //                         "iyr" => str_num_between(v, 2010, 2020),
    //                         "eyr" => str_num_between(v, 2020, 2030),
    //                         "hgt" => {
    //                             let n = &v[..v.len() - 2];
    //                             let units = &v[v.len() - 2..];
    //                             match units {
    //                                 "in" => str_num_between(n, 59, 76),
    //                                 "cm" => str_num_between(n, 150, 193),
    //                                 _ => false,
    //                             }
    //                         }
    //                         _ => true,
    //                     };

    //                 ok
    //             })
    //             .map(|(k, _)| k)
    //             .collect();

    //         let x = l;
    //         valid_keys.iter().unique().count() == rules.keys().len()
    //     })
    //     .count()
    0
}

fn str_num_between(s: &str, l: i32, u: i32) -> bool {
    let n = s.parse::<i32>().unwrap();
    l <= n && n <= u
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        ";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(input_transformer(SAMPLE));

        assert_eq!(2, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(input_transformer(&input));

        assert_eq!(196, solution);
    }

    #[test]
    fn test_part2_sample() {
        let input = "
            ok:yes
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

            ok:no
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        ";
        let solution = solve_part2(input_transformer(input));

        assert_eq!(4, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(input_transformer(&input));

        // 115
        assert_eq!(115, solution);
    }
}
