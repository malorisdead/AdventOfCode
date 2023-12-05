use std::str::FromStr;

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 5;
    const TITLE: &'static str = "If You Give A Seed A Fertilizer";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let almanac = self.input().get_as::<Almanac>()?;
        let mapped = almanac.map_seeds();
        Ok(*mapped.iter().min().unwrap())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mapping {
    source_min: usize,
    source_max: usize,
    dest_min: usize,
    dest_max: usize,
}
impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let specs = s.split(' ').filter_map(|x| x.parse().ok()).collect_vec();
        if specs.len() != 3 {
            return Err(anyhow!("Invalid range specification"));
        }
        let offset = specs[2] - 1;
        let dest_min = specs[0];
        let dest_max = dest_min + offset;
        let source_min = specs[1];
        let source_max = source_min + offset;

        Ok(Mapping {
            source_min,
            source_max,
            dest_min,
            dest_max,
        })
    }
}
impl Mapping {
    fn contains(&self, n: usize) -> bool {
        n >= self.source_min && n <= self.source_max
    }

    fn map(&self, n: usize) -> Option<usize> {
        match self.contains(n) {
            true => Some(self.dest_min + (n - self.source_min)),
            false => None,
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Vec<Mapping>>,
}
impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let sections = s.split("\n\n").collect_vec();
        if sections.len() < 2 {
            return Err(anyhow!("Invalid almanac"));
        }

        let seeds = sections[0]
            .split(' ')
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect_vec();

        let mut mappings = vec![];
        for i in 1..sections.len() {
            mappings.push(
                sections[i]
                    .lines()
                    .skip(1)
                    .filter_map(|l| l.parse::<Mapping>().ok())
                    .collect_vec(),
            );
        }

        Ok(Almanac { seeds, mappings })
    }
}
impl Almanac {
    fn map_seed(&self, seed: usize) -> usize {
        let mut mapped = seed;
        for group in &self.mappings {
            for map in group {
                match map.map(mapped) {
                    Some(m) => {
                        mapped = m;
                        break;
                    }
                    None => (),
                }
            }
        }
        mapped
    }

    fn map_seeds(&self) -> Vec<usize> {
        let mut ret = vec![];
        for &seed in &self.seeds {
            ret.push(self.map_seed(seed));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() -> Result<()> {
        let test = "seeds: 1 2 3\n\nseed-to-soil map:\n1 2 2";
        let almanac = test.parse::<Almanac>()?;
        let seeds = vec![1, 2, 3];
        let mappings = vec![vec![Mapping {
            source_min: 2,
            source_max: 3,
            dest_min: 1,
            dest_max: 2,
        }]];
        assert_eq!(seeds, almanac.seeds);
        assert_eq!(mappings, almanac.mappings);
        Ok(())
    }

    #[test]
    fn should_map() -> Result<()> {
        let test = "seeds: 1 2 3\n\nseed-to-soil map:\n1 2 2";
        let almanac = test.parse::<Almanac>()?;
        let expected = vec![1, 1, 2];
        let actual = almanac.map_seeds();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let almanac = EXAMPLE_INPUT.parse::<Almanac>()?;
        let mapped = almanac.map_seeds();
        assert_eq!(&35, mapped.iter().min().unwrap());
        Ok(())
    }

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
