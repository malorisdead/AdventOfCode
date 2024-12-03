use crate::solver::Solver;
use anyhow::Result;
use regex::Regex;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 03;
    const TITLE: &'static str = "Mull It Over";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let instructions = find_instructions(&self.input().get()?)?;
        Ok(execute(&instructions, true))
    }

    fn part_two(&self) -> Result<usize> {
        let instructions = find_instructions(&self.input().get()?)?;
        Ok(execute(&instructions, false))
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    DoNot,
}

fn find_instructions(s: &str) -> Result<Vec<Instruction>> {
    let mut instr = vec![];
    let finder = Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d+),(\d+)\))")?;
    for caps in finder.captures_iter(s) {
        match &caps[0] {
            "do()" => instr.push(Instruction::Do),
            "don't()" => instr.push(Instruction::DoNot),
            _ => {
                let left = &caps[1];
                let right = &caps[2];
                instr.push(Instruction::Mul(left.parse()?, right.parse()?));
            }
        }
    }
    Ok(instr)
}

fn execute(instr: &[Instruction], skip_conditionals: bool) -> usize {
    let mut do_mult = true;
    let mut total = 0;
    for i in instr {
        match i {
            Instruction::Mul(l, r) => {
                if skip_conditionals || do_mult {
                    total += l * r
                }
            }
            Instruction::Do => do_mult = true,
            Instruction::DoNot => do_mult = false,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn should_find_multiplication() -> Result<()> {
        let test = "xyzmul(12,34)abcmul(56,78)pdq";
        let found = find_instructions(test)?;
        assert_eq!(
            vec![Instruction::Mul(12, 34), Instruction::Mul(56, 78)],
            found
        );
        Ok(())
    }

    #[test]
    fn should_sum_results() -> Result<()> {
        let test = "xyzmul(1,2)abcmul(3,4)pdq";
        let found = find_instructions(test)?;
        let sum = execute(&found, true);
        assert_eq!(14, sum);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let instructions = find_instructions(EXAMPLE_INPUT)?;
        let sum = execute(&instructions, true);
        assert_eq!(161, sum);
        Ok(())
    }

    const EXAMPLE_INPUT_PART2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn should_find_other_instructions() -> Result<()> {
        let test = "abcdo()xyzdon't()";
        let found = find_instructions(test)?;
        assert_eq!(vec![Instruction::Do, Instruction::DoNot], found);
        Ok(())
    }

    #[test]
    fn should_skip_multiplication() -> Result<()> {
        let test = "mul(1,2)don't()mul(3,4)do()mul(5,6)";
        let ins = find_instructions(test)?;
        assert_eq!(
            vec![
                Instruction::Mul(1, 2),
                Instruction::DoNot,
                Instruction::Mul(3, 4),
                Instruction::Do,
                Instruction::Mul(5, 6)
            ],
            ins
        );

        let sum = execute(&ins, false);
        assert_eq!(32, sum);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let instructions = find_instructions(EXAMPLE_INPUT_PART2)?;
        let sum = execute(&instructions, false);
        assert_eq!(48, sum);
        Ok(())
    }
}
