use aoc::{read_input, Error};

fn main() -> Result<(), Error> {
    println!("Day 1!");

    let mut input = read_input::<i32>(2020, 1)?;

    match solve(&mut input) {
        Some(solution) => println!("Solution: {}", solution),
        _ => println!("No solution!"),
    };

    Ok(())
}

/// Solve 2020-01 challange
///
/// Time:   O(n^2)
/// Space:  O(1)
fn solve(input: &mut [i32]) -> Option<i32> {
    if input.len() < 2 {
        return None;
    }

    input.sort_unstable();

    match sum3n(&*input, 2020) {
        Some((i, j, k)) => Some(input[i] * input[j] * input[k]),
        None => None,
    }
}

/// Find two numbers that add to the `target_sum`
///
/// Assumes sorted `input`
///
/// Time:   O(n)
/// Space:  O(1)
fn sum2n(input: &[i32], target_sum: i32) -> Option<(usize, usize)> {
    let mut l = 0;
    let mut r = input.len() - 1;
    while l < r {
        match input[l] + input[r] {
            sum if sum == target_sum => return Some((l, r)),
            sum if sum < target_sum => l += 1,
            _ => r -= 1,
        }
    }

    None
}

/// Find three numbers that add to the `target_sum`
///
/// Assumes sorted `input`
///
/// Time:   O(n^2)
/// Space:  O(1)
fn sum3n(input: &[i32], target_sum: i32) -> Option<(usize, usize, usize)> {
    for k in 0..input.len() {
        match sum2n(input, target_sum - input[k]) {
            Some((i, j)) => return Some((i, j, k)),
            _ => continue,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use aoc::read_input;

    #[test]
    fn test_sample_input() {
        let input = &mut vec![1721, 979, 366, 299, 675, 1456];
        let solution = solve(input);

        assert_eq!(Some(241861950), solution);
    }

    #[test]
    fn test_empty_input() {
        let input = &mut vec![];
        let solution = solve(input);

        assert_eq!(None, solution);
    }

    #[test]
    fn test_no_solution() {
        let input = &mut vec![10, 2010];
        let solution = solve(input);

        assert_eq!(None, solution);
    }

    #[test]
    fn test_negative_numbers() {
        let input = &mut vec![-5, 2030];
        let solution = solve(input);

        assert_eq!(Some(50750), solution);
    }

    #[test]
    fn test_solution() {
        let mut input = read_input::<i32>(2020, 1).unwrap();
        let solution = solve(&mut input);
        assert_eq!(Some(155806250), solution);
    }
}
