use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::{anyhow, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 06;
    const TITLE: &'static str = "Guard Gallivant";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<LabMap>()?;
        map.walk_guard()
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<LabMap>()?;
        map.find_possible_obstructions()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn walk_guard(&self) -> Result<usize> {
        let path = do_walk(&self.tiles, self.guard_start)?;
        Ok(path.len())
    }

    fn find_possible_obstructions(&self) -> Result<usize> {
        let obstructions = find_possible_obstructions(&self.tiles, self.guard_start)?;
        Ok(obstructions.len())
    }
}

fn do_walk(
    tiles: &HashMap<IPoint, Tile>,
    start: IPoint,
) -> Result<HashMap<IPoint, HashSet<usize>>> {
    let mut visited = HashMap::new();
    let mut cur = start;
    let mut offset = 0;
    loop {
        let next = cur + OFFSETS[offset];
        let offsets = visited.entry(cur).or_insert(HashSet::new());
        if !offsets.insert(offset) {
            return Err(anyhow!("Caught in a loop!"));
        }
        if let Some(np) = tiles.get(&next) {
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
    Ok(visited)
}

fn find_possible_obstructions(tiles: &HashMap<IPoint, Tile>, start: IPoint) -> Result<Vec<IPoint>> {
    let mut obstructions = vec![];
    // Let's brute force it babey
    let optimal_path = do_walk(tiles, start)?;
    for &visited in optimal_path.keys() {
        if visited == start {
            continue;
        }
        let mut alt_tiles = tiles.clone();
        alt_tiles.entry(visited).and_modify(|t| *t = Tile::Blocked);
        if do_walk(&alt_tiles, start).is_err() {
            obstructions.push(visited);
        }
    }
    Ok(obstructions)
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
        let path = do_walk(&map.tiles, map.guard_start)?;
        assert_eq!(4, path.len());
        assert_eq!(
            HashMap::from([
                (IPoint::new(0, 2), HashSet::from([0])),
                (IPoint::new(0, 1), HashSet::from([0])),
                (IPoint::new(1, 1), HashSet::from([1])),
                (IPoint::new(1, 2), HashSet::from([2]))
            ]),
            path
        );
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<LabMap>()?;
        let path = map.walk_guard()?;
        assert_eq!(41, path);
        Ok(())
    }

    #[test]
    fn should_find_obstructions() -> Result<()> {
        let map = ".#..\n...#\n.^..\n..#.".parse::<LabMap>()?;
        let obstructions = find_possible_obstructions(&map.tiles, map.guard_start)?;
        assert_eq!(1, obstructions.len());
        assert_eq!(vec![IPoint::new(0, 2)], obstructions);
        Ok(())
    }
}
