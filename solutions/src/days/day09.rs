use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 09;
    const TITLE: &'static str = "Disk Fragmenter";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut fs = self.input().get_as::<Filesystem>()?;
        fs.compact();
        Ok(fs.checksum())
    }

    fn part_two(&self) -> Result<usize> {
        let mut fs = self.input().get_as::<Filesystem>()?;
        fs.smarter_compact();
        Ok(fs.checksum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    Free,
    File(u16),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct File {
    blocks: usize,
    start: usize,
}

struct Filesystem {
    // let's expand the compact, efficient description into a bloated huge one
    blocks: Vec<Block>,
    file_map: HashMap<u16, File>,
    max_id: u16,
}
impl FromStr for Filesystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = vec![];
        let mut file_map = HashMap::new();
        let mut file = true;
        let mut id = 0;
        for c in s.chars() {
            if !c.is_digit(10) {
                continue;
            }
            let len = c.to_digit(10).unwrap();
            for _ in 0..len {
                blocks.push(match file {
                    true => Block::File(id),
                    false => Block::Free,
                });
            }
            if file {
                file_map.insert(
                    id,
                    File {
                        blocks: len as usize,
                        start: blocks.len() - len as usize,
                    },
                );
                id += 1;
            }
            file = !file;
        }
        Ok(Self {
            blocks,
            file_map,
            max_id: id - 1,
        })
    }
}
impl Filesystem {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (i, block)| match block {
                Block::Free => acc,
                Block::File(id) => acc + i * *id as usize,
            })
    }

    fn compact(&mut self) {
        let (mut i, mut j) = (0, self.blocks.len() - 1);
        loop {
            while self.blocks[i] != Block::Free {
                i += 1;
            }
            while self.blocks[j] == Block::Free {
                j -= 1;
            }
            if i >= j {
                break;
            }
            self.blocks[i] = self.blocks[j];
            self.blocks[j] = Block::Free;
        }
    }

    fn find_free_space(&self, size: usize, max: usize) -> Option<usize> {
        let mut i = 0;
        loop {
            if i >= max {
                return None;
            }
            while self.blocks[i] != Block::Free {
                i += 1;
                if i >= max {
                    return None;
                }
            }
            let start = i;
            while i < max && self.blocks[i] == Block::Free {
                i += 1;
            }
            if i - start >= size {
                return Some(start);
            }
        }
    }

    fn smarter_compact(&mut self) {
        for id in (1..=self.max_id).rev() {
            let file = self.file_map.get(&id).unwrap();
            let Some(free_start) = self.find_free_space(file.blocks, file.start) else {
                continue;
            };
            if free_start > file.start {
                continue;
            }
            for i in 0..file.blocks {
                let (new, old) = (free_start + i, file.start + i);
                self.blocks[new] = self.blocks[old];
                self.blocks[old] = Block::Free;
            }
            self.file_map.entry(id).and_modify(|e| e.start = free_start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "123";
        let filesystem = example.parse::<Filesystem>()?;
        let expected = Filesystem {
            blocks: vec![
                Block::File(0),
                Block::Free,
                Block::Free,
                Block::File(1),
                Block::File(1),
                Block::File(1),
            ],
            file_map: HashMap::from([
                (
                    0,
                    File {
                        blocks: 1,
                        start: 0,
                    },
                ),
                (
                    1,
                    File {
                        blocks: 3,
                        start: 3,
                    },
                ),
            ]),
            max_id: 1,
        };
        assert_eq!(expected.blocks, filesystem.blocks);
        assert_eq!(expected.file_map, filesystem.file_map);
        assert_eq!(expected.max_id, filesystem.max_id);
        Ok(())
    }

    #[test]
    fn should_calculate_checksum() -> Result<()> {
        let filesystem = "113".parse::<Filesystem>()?;
        let checksum = filesystem.checksum();
        assert_eq!(9, checksum);
        Ok(())
    }

    #[test]
    fn should_compact() -> Result<()> {
        let mut fs = "12345".parse::<Filesystem>()?;
        fs.compact();
        assert_eq!(60, fs.checksum());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let mut fs = EXAMPLE_INPUT.parse::<Filesystem>()?;
        fs.compact();
        assert_eq!(1928, fs.checksum());
        Ok(())
    }

    #[test]
    fn should_find_free_space() -> Result<()> {
        let fs = "12345".parse::<Filesystem>()?;
        let free_start = fs.find_free_space(3, 12);
        assert_eq!(Some(6), free_start);
        Ok(())
    }

    #[test]
    fn should_smart_compact() -> Result<()> {
        let mut fs = "1234203".parse::<Filesystem>()?;
        fs.smarter_compact();
        assert_eq!(81, fs.checksum());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let mut fs = EXAMPLE_INPUT.parse::<Filesystem>()?;
        fs.smarter_compact();
        assert_eq!(2858, fs.checksum());
        Ok(())
    }
}
