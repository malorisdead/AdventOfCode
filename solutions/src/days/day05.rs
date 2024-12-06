use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    usize,
};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 05;
    const TITLE: &'static str = "Print Queue";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let order = self.input().get_as::<PrintOrder>()?;
        let checksum = order.get_valid_checksum();
        Ok(checksum)
    }

    fn part_two(&self) -> Result<usize> {
        let order = self.input().get_as::<PrintOrder>()?;
        let checksum = order.get_fixed_checksum();
        Ok(checksum)
    }
}

struct PrintOrder {
    precedence: HashMap<usize, HashSet<usize>>,
    pages: Vec<Vec<usize>>,
}
impl FromStr for PrintOrder {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut precedence = HashMap::new();
        let mut pages = vec![];
        let [order_rules, page_updates] = s.split("\n\n").collect_vec()[..2] else {
            return Err(anyhow!("Invalid print order specification"));
        };
        for l in order_rules.lines() {
            let [before, after] = l.split('|').collect_vec()[..2] else {
                return Err(anyhow!("Invalid page order rule"));
            };
            let (before, after): (usize, usize) = (before.parse()?, after.parse()?);
            let rules = precedence.entry(after).or_insert(HashSet::new());
            rules.insert(before);
        }
        for l in page_updates.lines() {
            pages.push(l.split(',').filter_map(|s| s.parse().ok()).collect_vec());
        }
        Ok(Self { precedence, pages })
    }
}
impl PrintOrder {
    fn get_valid_checksum(&self) -> usize {
        let mut checksum = 0;
        for update in &self.pages {
            if self.is_valid(update) {
                checksum += update[update.len() / 2];
            }
        }
        checksum
    }

    fn is_valid(&self, update: &[usize]) -> bool {
        let indexed: HashMap<usize, usize> =
            HashMap::from_iter(update.iter().enumerate().map(|(k, &v)| (v, k)));
        for (idx, page) in update.iter().enumerate() {
            let Some(precedents) = self.precedence.get(&page) else {
                continue;
            };
            for p in precedents {
                let Some(&pi) = indexed.get(p) else {
                    continue;
                };
                if pi > idx {
                    return false;
                }
            }
        }
        true
    }

    fn fix_page_order(&self) -> Vec<Vec<usize>> {
        let mut fixed_updates = vec![];
        for update in &self.pages {
            let mut needs_update = false;
            let mut updated = update.clone();
            let mut i = 0;
            // I was never good at sorting algorithms.
            // This is probably inefficient but it's what got me the star so...
            while i < updated.len() {
                let mut modified = false;
                let page = updated[i];
                let Some(precedents) = self.precedence.get(&page) else {
                    i += 1;
                    continue;
                };
                for j in (i + 1..updated.len()).rev() {
                    if precedents.contains(&updated[j]) {
                        modified = true;
                        updated.remove(i);
                        updated.insert(j, page);
                        break;
                    }
                }
                if !modified {
                    i += 1;
                } else {
                    needs_update = true;
                }
            }
            if needs_update {
                fixed_updates.push(updated);
            }
        }
        fixed_updates
    }

    fn get_fixed_checksum(&self) -> usize {
        let mut checksum = 0;
        for f in self.fix_page_order() {
            checksum += f[f.len() / 2];
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "1|2\n\n1,2";
        let exp_precedence = HashMap::from([(2, HashSet::from([1]))]);
        let exp_pages = vec![vec![1, 2]];
        let actual = example.parse::<PrintOrder>()?;
        assert_eq!(exp_precedence, actual.precedence);
        assert_eq!(exp_pages, actual.pages);
        Ok(())
    }

    #[test]
    fn should_be_valid() -> Result<()> {
        let order = "1|2\n\n1,2,3".parse::<PrintOrder>()?;
        let checksum = order.get_valid_checksum();
        assert_eq!(2, checksum);
        Ok(())
    }

    #[test]
    fn should_be_invalid() -> Result<()> {
        let order = "1|2\n\n3,2,1".parse::<PrintOrder>()?;
        let checksum = order.get_valid_checksum();
        assert_eq!(0, checksum);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let order = EXAMPLE_INPUT.parse::<PrintOrder>()?;
        let checksum = order.get_valid_checksum();
        assert_eq!(143, checksum);
        Ok(())
    }

    #[test]
    fn should_update_order() -> Result<()> {
        let order = "1|2\n2|3\n1|3\n\n3,2,1".parse::<PrintOrder>()?;
        let fixed = order.fix_page_order();
        assert_eq!(vec![vec![1, 2, 3]], fixed);
        Ok(())
    }

    #[test]
    fn should_get_fixed_checksum() -> Result<()> {
        let order = "1|2\n2|3\n1|3\n\n3,2,1\n1,3,2".parse::<PrintOrder>()?;
        let checksum = order.get_fixed_checksum();
        assert_eq!(4, checksum);
        Ok(())
    }

    #[test]
    fn should_skip_valid() -> Result<()> {
        let order = "1|2\n2|3\n1|3\n\n3,2,1\n1,2,3".parse::<PrintOrder>()?;
        let checksum = order.get_fixed_checksum();
        assert_eq!(2, checksum);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let order = EXAMPLE_INPUT.parse::<PrintOrder>()?;
        let checksum = order.get_fixed_checksum();
        assert_eq!(123, checksum);
        Ok(())
    }
}
