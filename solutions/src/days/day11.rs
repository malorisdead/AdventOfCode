use std::collections::HashMap;

use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 11;
    const TITLE: &'static str = "Plutonian Pebbles";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut stones = self.input().get_split_as::<usize>(' ')?;
        for _ in 0..25 {
            stones = blink(&stones);
        }
        Ok(stones.len())
    }

    fn part_two(&self) -> Result<usize> {
        let stones = self.input().get_split_as::<usize>(' ')?;
        Ok(blink_smarter_not_harder(&stones, 75))
    }
}

fn blink(stones: &[usize]) -> Vec<usize> {
    let mut output = vec![];
    for &s in stones {
        if s == 0 {
            output.push(1);
        } else if (s.ilog10() + 1) % 2 == 0 {
            let split = 10usize.pow((s.ilog10() + 1) / 2);
            let left = s / split;
            let right = s - (left * split);
            output.push(left);
            output.push(right);
        } else {
            output.push(s * 2024);
        }
    }
    output
}

fn blink_smarter_not_harder(stones: &[usize], iterations: usize) -> usize {
    let mut stone_counts: HashMap<usize, usize> =
        HashMap::from_iter(stones.iter().map(|&s| (s, 1usize)));
    let mut new_counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..iterations {
        for (&k, &num) in &stone_counts {
            if k == 0 {
                *new_counts.entry(1).or_insert(0) += num;
            } else if (k.ilog10() + 1) % 2 == 0 {
                let split = 10usize.pow((k.ilog10() + 1) / 2);
                let left = k / split;
                let right = k - (left * split);
                *new_counts.entry(left).or_insert(0) += num;
                *new_counts.entry(right).or_insert(0) += num;
            } else {
                *new_counts.entry(k * 2024).or_insert(0) += num;
            }
        }
        // This should still take less memory than brute-forcing this
        stone_counts = new_counts.clone();
        new_counts.clear();
    }
    stone_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_blink() {
        let test = vec![0, 1, 24];
        let expected = vec![1, 2024, 2, 4];
        let actual = blink(&test);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_solve_part1() {
        let mut test = vec![125, 17];
        for _ in 0..25 {
            test = blink(&test);
        }
        assert_eq!(55312, test.len());
    }

    #[test]
    fn should_work_smarter() {
        let test = vec![125, 17];
        let stone_count = blink_smarter_not_harder(&test, 25);
        assert_eq!(55312, stone_count);
    }
}
