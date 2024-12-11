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
        let mut stones = self.input().get_split_as::<usize>(' ')?;
        for _ in 0..75 {
            stones = blink(&stones);
        }
        Ok(stones.len())
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
}
