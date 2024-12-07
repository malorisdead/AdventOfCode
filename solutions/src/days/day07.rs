use std::str::FromStr;

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 07;
    const TITLE: &'static str = "Bridge Repair";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let cals = self.input().get_lines_as::<Calibration>()?;
        Ok(valid_checksum(&cals, false))
    }

    fn part_two(&self) -> Result<usize> {
        let cals = self.input().get_lines_as::<Calibration>()?;
        Ok(valid_checksum(&cals, true))
    }
}

enum Operators {
    Add,
    Mult,
    Cat,
}
const OPS_NO_CAT: [Operators; 2] = [Operators::Add, Operators::Mult];
const OPS_WITH_CAT: [Operators; 3] = [Operators::Add, Operators::Mult, Operators::Cat];

// Call Garrus, we've got some calibrations to do!
struct Calibration {
    result: usize,
    operands: Vec<usize>,
}
impl FromStr for Calibration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [left, right] = s.split(": ").collect_vec()[..2] else {
            return Err(anyhow!("Invalid calibration"));
        };
        let result = left.parse()?;
        let operands = right.split(' ').filter_map(|x| x.parse().ok()).collect();
        Ok(Self { result, operands })
    }
}
impl Calibration {
    fn is_computable(&self, concat: bool) -> bool {
        let ops = if concat {
            &OPS_WITH_CAT[..]
        } else {
            &OPS_NO_CAT[..]
        };
        let mut results = vec![self.operands[0]];
        for i in 1..self.operands.len() {
            let mut next_results = vec![];
            let next = self.operands[i];
            for &prev in &results {
                for o in ops {
                    let possibility = match o {
                        Operators::Add => prev + next,
                        Operators::Mult => prev * next,
                        Operators::Cat => (prev * 10usize.pow(next.ilog10() + 1)) + next,
                    };
                    if possibility <= self.result {
                        next_results.push(possibility);
                    }
                }
            }
            results = next_results;
        }
        results.contains(&self.result)
    }
}

fn valid_checksum(cals: &[Calibration], concat: bool) -> usize {
    cals.iter()
        .filter_map(|c| c.is_computable(concat).then_some(c.result))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "123: 1 2 3";
        let calibration = test.parse::<Calibration>()?;
        assert_eq!(123, calibration.result);
        assert_eq!(vec![1, 2, 3], calibration.operands);
        Ok(())
    }

    #[test]
    fn should_be_computable() -> Result<()> {
        let cal = "24: 2 3 4".parse::<Calibration>()?;
        assert!(cal.is_computable(false));
        Ok(())
    }

    #[test]
    fn should_not_be_computable() -> Result<()> {
        let cal = "24: 1 2 3".parse::<Calibration>()?;
        assert!(!cal.is_computable(false));
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let cals = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Calibration>().ok())
            .collect_vec();
        let checksum = valid_checksum(&cals, false);
        assert_eq!(3749, checksum);
        Ok(())
    }

    #[test]
    fn should_be_concatenated() -> Result<()> {
        let cal = "12: 1 2".parse::<Calibration>()?;
        assert!(cal.is_computable(true));
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let cals = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Calibration>().ok())
            .collect_vec();
        let checksum = valid_checksum(&cals, true);
        assert_eq!(11387, checksum);
        Ok(())
    }
}
