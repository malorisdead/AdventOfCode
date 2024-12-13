use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 12;
    const TITLE: &'static str = "Garden Groups";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let garden = self.input().get_as::<Garden>()?;
        Ok(garden.get_fence_cost())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

const OFFSETS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

struct Region {
    identifier: char,
    perimeter: usize,
    points: HashSet<IPoint>,
}
impl Region {
    fn new(identifier: char) -> Self {
        Self {
            identifier,
            perimeter: 0,
            points: HashSet::new(),
        }
    }
}

struct Garden {
    tiles: HashMap<IPoint, char>,
    max_x: isize,
    max_y: isize,
}
impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, l) in s.lines().enumerate() {
            let y = y as isize;
            for (x, c) in l.char_indices() {
                let x = x as isize;
                tiles.insert(IPoint::new(x, y), c);
                if x > max_x {
                    max_x = x;
                }
            }
            max_y = y;
        }
        Ok(Self {
            tiles,
            max_x,
            max_y,
        })
    }
}
impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut visited: HashSet<IPoint> = HashSet::new();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let p = IPoint::new(x, y);
                if visited.contains(&p) {
                    continue;
                }
                let mut region = Region::new(self.tiles[&p]);
                self.walk_region(p, &mut region, &mut visited);
                regions.push(region);
            }
        }
        regions
    }

    fn walk_region(&self, start: IPoint, region: &mut Region, visited: &mut HashSet<IPoint>) {
        if !visited.insert(start) {
            return;
        }
        if !region.points.insert(start) {
            return;
        }
        for o in &OFFSETS {
            let ap = start + o;
            if region.points.contains(&ap) {
                continue;
            }
            let Some(adj) = self.tiles.get(&ap) else {
                region.perimeter += 1;
                continue;
            };
            if adj != &region.identifier {
                region.perimeter += 1;
                continue;
            }
            self.walk_region(ap, region, visited);
        }
    }

    fn get_fence_cost(&self) -> usize {
        let regions = self.get_regions();
        regions.iter().map(|r| r.points.len() * r.perimeter).sum()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    const EXAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "AA\nBB";
        let expected = Garden {
            tiles: HashMap::from([
                (IPoint::new(0, 0), 'A'),
                (IPoint::new(1, 0), 'A'),
                (IPoint::new(0, 1), 'B'),
                (IPoint::new(1, 1), 'B'),
            ]),
            max_x: 1,
            max_y: 1,
        };
        let actual = example.parse::<Garden>()?;
        assert_eq!(expected.tiles, actual.tiles);
        assert_eq!(expected.max_x, actual.max_x);
        assert_eq!(expected.max_y, actual.max_y);
        Ok(())
    }

    #[test]
    fn should_get_regions() -> Result<()> {
        let garden = "AA\nBB".parse::<Garden>()?;
        let regions = garden.get_regions();
        assert_eq!(2, regions.len());
        let [a, b] = &regions[..2] else {
            return Err(anyhow!("Invalid length!"));
        };
        assert_eq!('A', a.identifier);
        assert_eq!(6, a.perimeter);
        assert_eq!('B', b.identifier);
        assert_eq!(6, b.perimeter);
        Ok(())
    }

    #[test]
    fn should_get_fencing_cost() -> Result<()> {
        let garden = "AA\nBB".parse::<Garden>()?;
        assert_eq!(24, garden.get_fence_cost());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let garden = EXAMPLE_INPUT.parse::<Garden>()?;
        assert_eq!(1930, garden.get_fence_cost());
        Ok(())
    }
}
