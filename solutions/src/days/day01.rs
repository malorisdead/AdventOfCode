use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 1;
    const TITLE: &'static str = "Historian Hysteria";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let lists = self.input().get_as::<LocationLists>()?;
        Ok(lists.get_total_distance())
    }

    fn part_two(&self) -> Result<isize> {
        let lists = self.input().get_as::<LocationLists>()?;
        Ok(lists.get_similarity_score())
    }
}

struct LocationLists {
    left: Vec<isize>,
    right: Vec<isize>,
}
impl FromStr for LocationLists {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut left = vec![];
        let mut right = vec![];

        for l in s.lines() {
            let split = l.split_whitespace().collect_vec();
            if split.len() != 2 {
                return Err(anyhow!("Invalid list pair"));
            }
            left.push(split[0].parse()?);
            right.push(split[1].parse()?);
        }

        Ok(Self { left, right })
    }
}
impl LocationLists {
    fn get_total_distance(&self) -> isize {
        self.left
            .iter()
            .sorted()
            .zip(self.right.iter().sorted())
            .fold(0, |acc, (&l, &r)| acc + (l - r).abs())
    }

    fn get_similarity_score(&self) -> isize {
        let mut score = 0;
        let mut counts: HashMap<&isize, isize> = HashMap::new();
        for r in &self.right {
            *counts.entry(r).or_insert(0) += 1;
        }
        for l in &self.left {
            score += l * counts.get(l).unwrap_or(&0);
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "0    1";
        let test = example.parse::<LocationLists>()?;
        assert_eq!(1, test.left.len());
        assert_eq!(1, test.right.len());
        assert_eq!(0, test.left[0]);
        assert_eq!(1, test.right[0]);
        Ok(())
    }

    #[test]
    fn should_get_distance() -> Result<()> {
        let example = "1    3\n7    4";
        let test = example.parse::<LocationLists>()?;
        let distance = test.get_total_distance();
        assert_eq!(5, distance);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<LocationLists>()?;
        assert_eq!(11, test.get_total_distance());
        Ok(())
    }

    #[test]
    fn should_calculate_score() -> Result<()> {
        let example = "1 2\n2 3\n3 2";
        let test = example.parse::<LocationLists>()?;
        assert_eq!(7, test.get_similarity_score());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<LocationLists>()?;
        assert_eq!(31, test.get_similarity_score());
        Ok(())
    }
}
