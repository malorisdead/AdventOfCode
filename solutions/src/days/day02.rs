use std::str::FromStr;

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 2;
    const TITLE: &'static str = "Red-Nosed Reports";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let reports = self.input().get_lines_as::<Report>()?;
        Ok(reports.iter().filter(|r| r.is_safe()).count())
    }

    fn part_two(&self) -> Result<usize> {
        let reports = self.input().get_lines_as::<Report>()?;
        Ok(reports.iter().filter(|r| r.is_safe_dampened()).count())
    }
}

enum LevelState {
    Unsafe,
    Safe(isize),
}

struct Report {
    levels: Vec<isize>,
}
impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect_vec();
        Ok(Self { levels })
    }
}
impl Report {
    fn is_safe(&self) -> bool {
        are_safe(&self.levels)
    }

    fn is_safe_dampened(&self) -> bool {
        // This is what you get when the too-clever solution doesn't work for all edge cases
        if are_safe(&self.levels) {
            return true;
        }
        for n in 0..self.levels.len() {
            let skipped: Vec<_> = self
                .levels
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (i != n).then_some(e))
                .collect();
            if are_safe(&skipped) {
                return true;
            }
        }
        false
    }
}

fn are_safe(levels: &[isize]) -> bool {
    let mut sign = 0;
    let max = levels.len() - 1;
    for i in 0..max {
        let left = levels[i];
        let right = levels[i + 1];
        match check_diff(left, right, sign) {
            LevelState::Unsafe => return false,
            LevelState::Safe(ds) => {
                if sign == 0 {
                    sign = ds;
                }
            }
        }
    }
    true
}

fn check_diff(left: isize, right: isize, sign: isize) -> LevelState {
    let diff = right - left;
    let d_sign = diff.signum();
    match diff.abs() {
        1..=3 => (),
        _ => return LevelState::Unsafe,
    }
    if sign != 0 && sign != d_sign {
        return LevelState::Unsafe;
    }
    LevelState::Safe(d_sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "1 2 3 4 5";
        let report = example.parse::<Report>()?;
        assert_eq!(vec![1isize, 2, 3, 4, 5], report.levels);
        Ok(())
    }

    #[test]
    fn should_be_safe() -> Result<()> {
        let test = "1 2 3 4 5".parse::<Report>()?;
        assert_eq!(true, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_be_safe_negative() -> Result<()> {
        let test = "5 4 3 2 1".parse::<Report>()?;
        assert_eq!(true, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_detect_large_increase() -> Result<()> {
        let test = "1 10".parse::<Report>()?;
        assert_eq!(false, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_detect_large_decrease() -> Result<()> {
        let test = "999 222".parse::<Report>()?;
        assert_eq!(false, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_detect_change_in_direction() -> Result<()> {
        let test = "3 2 1 2 3".parse::<Report>()?;
        assert_eq!(false, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_detect_no_change() -> Result<()> {
        let test = "1 2 3 3".parse::<Report>()?;
        assert_eq!(false, test.is_safe());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let reports = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Report>().ok())
            .collect_vec();
        let safe_count = reports.iter().filter(|r| r.is_safe()).count();
        assert_eq!(2, safe_count);
        Ok(())
    }

    #[test]
    fn should_be_safe_dampened() -> Result<()> {
        let test = "3 1 2 4 6 9 10".parse::<Report>()?;
        assert_eq!(true, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_be_safe_dampened_change_direction() -> Result<()> {
        let test = "1 2 1 3".parse::<Report>()?;
        assert_eq!(true, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_be_safe_dampened_no_change() -> Result<()> {
        let test = "1 2 3 3".parse::<Report>()?;
        assert_eq!(true, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_detect_multiple_errors() -> Result<()> {
        let test = "1 2 5 3 4 3".parse::<Report>()?;
        assert_eq!(false, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_detect_jump_at_start() -> Result<()> {
        let test = "1 5 6 7 8".parse::<Report>()?;
        assert_eq!(true, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_detect_multiple_errors_at_start() -> Result<()> {
        let test = "1 8 1 9 10".parse::<Report>()?;
        assert_eq!(false, test.is_safe_dampened());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let reports = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Report>().ok())
            .collect_vec();
        let dampened_count = reports.iter().filter(|r| r.is_safe_dampened()).count();
        assert_eq!(4, dampened_count);
        Ok(())
    }
}
