use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 06;
    const TITLE: &'static str = "UNKNOWN";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<LabMap>()?;
        Ok(map.walk_guard().len())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Clear,
    Blocked,
}

const OFFSETS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct LabMap {
    tiles: HashMap<IPoint, Tile>,
    guard_start: IPoint,
}
impl FromStr for LabMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let mut guard_start = IPoint::new(0, 0);
        for (y, l) in s.lines().enumerate() {
            let y = y as isize;
            for (x, c) in l.chars().enumerate() {
                let x = x as isize;
                let p = IPoint::new(x, y);
                let _ = match c {
                    '^' => {
                        guard_start = p;
                        tiles.insert(p, Tile::Clear)
                    }
                    '#' => tiles.insert(p, Tile::Blocked),
                    _ => tiles.insert(p, Tile::Clear),
                };
            }
        }
        Ok(Self { tiles, guard_start })
    }
}
impl LabMap {
    fn walk_guard(&self) -> HashSet<IPoint> {
        let mut visited = HashSet::new();
        let mut cur = self.guard_start;
        let mut offset = 0;
        loop {
            let next = cur + OFFSETS[offset];
            visited.insert(cur);
            if let Some(np) = self.tiles.get(&next) {
                match np {
                    Tile::Blocked => {
                        offset = (offset + 1) % 4;
                        continue;
                    }
                    Tile::Clear => (),
                }
                cur = next;
            } else {
                break;
            }
        }
        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn should_parse() -> Result<()> {
        let example = ".#\n^.";
        let expected = LabMap {
            tiles: HashMap::from([
                (IPoint::new(0, 0), Tile::Clear),
                (IPoint::new(1, 0), Tile::Blocked),
                (IPoint::new(0, 1), Tile::Clear),
                (IPoint::new(1, 1), Tile::Clear),
            ]),
            guard_start: IPoint::new(0, 1),
        };
        let actual = example.parse::<LabMap>()?;
        assert_eq!(expected.tiles, actual.tiles);
        assert_eq!(expected.guard_start, actual.guard_start);
        Ok(())
    }

    #[test]
    fn should_walk_path() -> Result<()> {
        let map = "#..\n..#\n^..".parse::<LabMap>()?;
        let path = map.walk_guard();
        assert_eq!(4, path.len());
        assert_eq!(
            HashSet::from([
                IPoint::new(0, 2),
                IPoint::new(0, 1),
                IPoint::new(1, 1),
                IPoint::new(1, 2)
            ]),
            path
        );
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<LabMap>()?;
        let path = map.walk_guard();
        assert_eq!(41, path.len());
        Ok(())
    }
}
