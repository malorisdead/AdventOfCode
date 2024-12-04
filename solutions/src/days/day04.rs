use std::{collections::HashMap, str::FromStr};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 04;
    const TITLE: &'static str = "Ceres Search";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let word_search = self.input().get_as::<WordSearch>()?;
        Ok(word_search.count_occurrences("XMAS"))
    }

    fn part_two(&self) -> Result<usize> {
        let word_search = self.input().get_as::<WordSearch>()?;
        Ok(word_search.count_x_mas())
    }
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
const CROSS_OFFSETS: [(isize, isize); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

#[derive(Debug)]
struct WordSearch {
    max_x: isize,
    max_y: isize,
    letters: HashMap<IPoint, char>,
}
impl FromStr for WordSearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut letters = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                letters.insert(IPoint::new(x, y), c);
                if x > max_x {
                    max_x = x;
                }
            }
            max_y = y as isize;
        }
        Ok(Self {
            letters,
            max_x,
            max_y,
        })
    }
}
impl WordSearch {
    fn count_occurrences(&self, word: &str) -> usize {
        let mut count = 0;
        let chars = word.chars().collect_vec();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let p = IPoint::new(x, y);
                if let Some(c) = self.letters.get(&p) {
                    if c == &chars[0] {
                        for offset in OFFSETS {
                            if self.search(p + offset, offset, &chars[1..]) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    fn search(&self, start: IPoint, offset: (isize, isize), needle: &[char]) -> bool {
        if needle.len() == 0 {
            // we've run out of characters to look for, so we must have found the word!
            return true;
        }
        if let Some(c) = self.letters.get(&start) {
            if c == &needle[0] {
                return self.search(start + offset, offset, &needle[1..]);
            }
        }
        false
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let p = IPoint::new(x, y);
                if let Some(c) = self.letters.get(&p) {
                    if c == &'A' {
                        let mut mas_count = 0;
                        for o in CROSS_OFFSETS {
                            if self.search(p + o, (o.0 * -1, o.1 * -1), &XMAS[1..]) {
                                mas_count += 1;
                            }
                        }
                        if mas_count == 2 {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "XMAS\nSAMX";
        let expected = HashMap::from([
            (IPoint::new(0, 0), 'X'),
            (IPoint::new(1, 0), 'M'),
            (IPoint::new(2, 0), 'A'),
            (IPoint::new(3, 0), 'S'),
            (IPoint::new(0, 1), 'S'),
            (IPoint::new(1, 1), 'A'),
            (IPoint::new(2, 1), 'M'),
            (IPoint::new(3, 1), 'X'),
        ]);
        let actual = example.parse::<WordSearch>()?;
        assert_eq!(expected, actual.letters);
        assert_eq!(3, actual.max_x);
        assert_eq!(1, actual.max_y);
        Ok(())
    }

    #[test]
    fn should_find_word() -> Result<()> {
        let example = "XMAS\nSAMX";
        let word_search = example.parse::<WordSearch>()?;
        let count = word_search.count_occurrences("XMAS");
        assert_eq!(2, count);
        Ok(())
    }

    #[test]
    fn should_find_in_all_directions() -> Result<()> {
        let big_example = "S..S..S\n.A.A.A.\n..MMM..\nSAMXMAS\n..MMM..\n.A.A.A.\nS..S..S";
        let word_search = big_example.parse::<WordSearch>()?;
        let count = word_search.count_occurrences("XMAS");
        assert_eq!(8, count);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let search = EXAMPLE_INPUT.parse::<WordSearch>()?;
        let count = search.count_occurrences("XMAS");
        assert_eq!(18, count);
        Ok(())
    }

    #[test]
    fn should_find_x_mas() -> Result<()> {
        let example = "M.S\n.A.\nM.S";
        let search = example.parse::<WordSearch>()?;
        let count = search.count_x_mas();
        assert_eq!(1, count);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let search = EXAMPLE_INPUT.parse::<WordSearch>()?;
        let count = search.count_x_mas();
        assert_eq!(9, count);
        Ok(())
    }
}
