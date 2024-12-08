use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 08;
    const TITLE: &'static str = "Resonant Collinearity";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        let antinodes = map.find_antinodes();
        Ok(antinodes.len())
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        let antinodes = map.find_resonant_antinodes();
        Ok(antinodes.len())
    }
}

struct Map {
    antennas: HashMap<char, Vec<IPoint>>,
    max_x: isize,
    max_y: isize,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, l) in s.lines().enumerate() {
            let y = y as isize;
            for (x, c) in l.chars().enumerate() {
                let x = x as isize;
                match c {
                    '.' => (),
                    _ => antennas.entry(c).or_insert(vec![]).push(IPoint::new(x, y)),
                }
                if x > max_x {
                    max_x = x;
                }
            }
            max_y = y;
        }
        Ok(Self {
            antennas,
            max_x,
            max_y,
        })
    }
}
impl Map {
    fn find_antinodes(&self) -> HashSet<IPoint> {
        let mut antinodes = HashSet::new();
        for freq_list in self.antennas.values() {
            if freq_list.len() < 2 {
                continue;
            }
            for i in 0..(freq_list.len() - 1) {
                for j in (i + 1)..freq_list.len() {
                    let (left, right) = (freq_list[i], freq_list[j]);
                    let dist = right - left;
                    for antinode in [left - dist, right + dist] {
                        if antinode.x >= 0
                            && antinode.x <= self.max_x
                            && antinode.y >= 0
                            && antinode.y <= self.max_y
                        {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }
        antinodes
    }

    fn find_resonant_antinodes(&self) -> HashSet<IPoint> {
        let mut antinodes = HashSet::new();
        for freq_list in self.antennas.values() {
            if freq_list.len() < 2 {
                continue;
            }
            for i in 0..(freq_list.len() - 1) {
                for j in (i + 1)..freq_list.len() {
                    let (left, right) = (freq_list[i], freq_list[j]);
                    antinodes.insert(left);
                    antinodes.insert(right);
                    let dist = right - left;
                    for is_forward in [false, true] {
                        let mut cur = if is_forward { left } else { right };
                        loop {
                            cur = if is_forward { cur - dist } else { cur + dist };
                            if cur.x < 0 || cur.y < 0 || cur.x > self.max_x || cur.y > self.max_y {
                                break;
                            }
                            antinodes.insert(cur);
                        }
                    }
                }
            }
        }
        antinodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "..a.\n.b..\na...";
        let expected = Map {
            antennas: HashMap::from([
                ('a', vec![IPoint::new(2, 0), IPoint::new(0, 2)]),
                ('b', vec![IPoint::new(1, 1)]),
            ]),
            max_x: 3,
            max_y: 2,
        };
        let actual = example.parse::<Map>()?;
        assert_eq!(expected.antennas, actual.antennas);
        assert_eq!(expected.max_x, actual.max_x);
        assert_eq!(expected.max_y, actual.max_y);
        Ok(())
    }

    #[test]
    fn should_find_antinodes() -> Result<()> {
        let map = "......\n......\n..a...\n......\n...a..\n......\n......".parse::<Map>()?;
        let expected = HashSet::from([IPoint::new(1, 0), IPoint::new(4, 6)]);
        let antinodes = map.find_antinodes();
        assert_eq!(expected, antinodes);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<Map>()?;
        let antinodes = map.find_antinodes();
        assert_eq!(14, antinodes.len());
        Ok(())
    }

    #[test]
    fn should_find_resonant_antinodes() -> Result<()> {
        let map = "T.........\n...T......\n.T........\n..........\n..........\n..........\n..........\n..........\n..........".parse::<Map>()?;
        let antinodes = map.find_resonant_antinodes();
        let expected = HashSet::from([
            IPoint::new(0, 0),
            IPoint::new(3, 1),
            IPoint::new(1, 2),
            IPoint::new(5, 0),
            IPoint::new(6, 2),
            IPoint::new(9, 3),
            IPoint::new(2, 4),
            IPoint::new(3, 6),
            IPoint::new(4, 8),
        ]);
        assert_eq!(expected, antinodes);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<Map>()?;
        let antinodes = map.find_resonant_antinodes();
        assert_eq!(34, antinodes.len());
        Ok(())
    }
}
