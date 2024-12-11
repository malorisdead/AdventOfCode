use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 10;
    const TITLE: &'static str = "Hoof It";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<TrailMap>()?;
        Ok(map.get_hiking_score())
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<TrailMap>()?;
        Ok(map.get_rating())
    }
}

const OFFSETS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

struct TrailMap {
    tiles: HashMap<IPoint, u8>,
    trailheads: Vec<IPoint>,
}
impl FromStr for TrailMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let mut trailheads = vec![];
        for (y, l) in s.lines().enumerate() {
            let y = y as isize;
            for (x, c) in l.chars().enumerate() {
                if !c.is_digit(10) {
                    continue;
                }
                let x = x as isize;
                let p = IPoint::new(x, y);
                let v = c.to_digit(10).unwrap().try_into()?;
                if v == 0 {
                    trailheads.push(p);
                }
                tiles.insert(p, v);
            }
        }
        Ok(Self { tiles, trailheads })
    }
}
impl TrailMap {
    fn get_hiking_score(&self) -> usize {
        let mut score = 0;
        for th in &self.trailheads {
            let mut endpoints = HashSet::new();
            self.find_endpoints(th, &0, &mut endpoints);
            score += endpoints.len();
        }
        score
    }

    fn find_endpoints(&self, start: &IPoint, value: &u8, endpoints: &mut HashSet<IPoint>) {
        if *value == 9 {
            endpoints.insert(*start);
            return;
        }
        for offset in &OFFSETS {
            let ap = start + offset;
            let Some(&adjacent) = self.tiles.get(&ap) else {
                continue;
            };
            if adjacent != value + 1 {
                continue;
            }
            self.find_endpoints(&ap, &adjacent, endpoints);
        }
    }

    fn get_rating(&self) -> usize {
        let mut rating = 0;
        let mut visited = HashMap::new();
        for th in &self.trailheads {
            rating += self.count_trails(th, &0, &mut visited);
        }
        rating
    }

    // lol this is what I had for my first attempt at a solution for part 1
    // before I realized I was counting each possible branch, not each endpoint
    fn count_trails(
        &self,
        start: &IPoint,
        value: &u8,
        visited: &mut HashMap<IPoint, usize>,
    ) -> usize {
        if *value == 9 {
            visited.insert(*start, 1);
            return 1;
        }
        let mut rating = 0;
        for offset in &OFFSETS {
            let ap = start + offset;
            let Some(&adjacent) = self.tiles.get(&ap) else {
                continue;
            };
            if adjacent != value + 1 {
                continue;
            }
            if let Some(v) = visited.get(&ap) {
                rating += v;
            } else {
                rating += self.count_trails(&ap, &adjacent, visited);
            }
        }
        rating
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "01\n23";
        let actual = example.parse::<TrailMap>()?;
        let expected = TrailMap {
            tiles: HashMap::from([
                (IPoint::new(0, 0), 0),
                (IPoint::new(1, 0), 1),
                (IPoint::new(0, 1), 2),
                (IPoint::new(1, 1), 3),
            ]),
            trailheads: vec![IPoint::new(0, 0)],
        };
        assert_eq!(expected.tiles, actual.tiles);
        assert_eq!(expected.trailheads, actual.trailheads);
        Ok(())
    }

    #[test]
    fn should_calculate_score() -> Result<()> {
        let map = "0123\n1234\n8765\n9876".parse::<TrailMap>()?;
        let score = map.get_hiking_score();
        assert_eq!(1, score);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<TrailMap>()?;
        let score = map.get_hiking_score();
        assert_eq!(36, score);
        Ok(())
    }

    #[test]
    fn should_get_rating() -> Result<()> {
        let map = "0123\n1234\n8765\n9876".parse::<TrailMap>()?;
        let rating = map.get_rating();
        assert_eq!(16, rating);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<TrailMap>()?;
        let score = map.get_rating();
        assert_eq!(81, score);
        Ok(())
    }
}
