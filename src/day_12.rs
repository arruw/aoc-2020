const DAY: u32 = 12;

type Input = Vec<(char, i32)>;
type Output = i32;

fn input_transformer(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            let mut action = l.chars().next().unwrap();
            let mut value = l[1..].parse::<i32>().unwrap();
            if action == 'L' || action == 'R' {
                value %= 360;
            }
            if action == 'L' {
                action = 'R';
                value = 360 - value;
            }
            (action, value)
        })
        .collect()
}

fn solve_part1(input: &Input) -> Output {
    let mut position: (i32, i32, i32) = (0, 0, 90);
    for (action, value) in input {
        match action {
            'N' => position.1 += *value,
            'S' => position.1 -= *value,
            'E' => position.0 += *value,
            'W' => position.0 -= *value,
            'R' => position.2 = (360 + position.2 + value) % 360,
            'F' => match position.2 {
                0 => position.1 += *value,
                180 => position.1 -= *value,
                90 => position.0 += *value,
                270 => position.0 -= *value,
                _ => panic!("Invalid action '{}{}' @{:?}!", action, value, position),
            },
            _ => panic!("Invalid action '{}{}' @{:?}!", action, value, position),
        }
    }

    position.0.abs() + position.1.abs()
}

fn solve_part2(input: &Input) -> Output {
    let mut waypoint: (i32, i32) = (10, 1);
    let mut position: (i32, i32) = (0, 0);
    for (action, value) in input {
        match action {
            'N' => waypoint.1 += *value,
            'S' => waypoint.1 -= *value,
            'E' => waypoint.0 += *value,
            'W' => waypoint.0 -= *value,
            'R' => match value {
                0 => (),
                90 => waypoint = (waypoint.1, -waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (-waypoint.1, waypoint.0),
                _ => panic!(
                    "Invalid action '{}{}' @{:?}>{:?}!",
                    action, value, position, waypoint
                ),
            },
            'F' => {
                position.0 += waypoint.0 * *value;
                position.1 += waypoint.1 * *value;
            }
            _ => panic!(
                "Invalid action '{}{}' @{:?}>{:?}!",
                action, value, position, waypoint
            ),
        }
    }

    position.0.abs() + position.1.abs()
}

#[cfg(test)]
mod tests {
    use super::{input_transformer, solve_part1, solve_part2, DAY};
    use crate::utils::*;

    const SAMPLE: &str = "F10
    N3
    S5
    E4
    W9
    E5
    N5
    F7
    R270
    L180
    R90
    L90
    F11";

    #[test]
    fn test_part1_sample() {
        let solution = solve_part1(&input_transformer(SAMPLE));

        assert_eq!(25, solution);
    }

    #[test]
    fn test_part1_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part1(&input_transformer(&input));

        assert_eq!(1687, solution);
    }

    #[test]
    fn test_part2_sample() {
        let solution = solve_part2(&input_transformer(SAMPLE));

        assert_eq!(286, solution);
    }

    #[test]
    fn test_part2_puzzle() {
        let input = read_input(2020, DAY).unwrap();
        let solution = solve_part2(&input_transformer(&input));

        assert_eq!(20873, solution);
    }
}
