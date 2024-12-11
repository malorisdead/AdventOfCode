// Stop warning me about unused code in this file damnit
#![allow(dead_code)]

use anyhow::Result;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub struct AdventInput {
    file: PathBuf,
}
impl AdventInput {
    pub fn for_day(day: u8) -> AdventInput {
        let file = PathBuf::from(format!("inputs/day{:02}.txt", day));
        AdventInput { file }
    }

    pub fn get(&self) -> Result<String> {
        Ok(fs::read_to_string(&self.file)?)
    }

    pub fn get_as<T>(&self) -> Result<T, T::Err>
    where
        T: FromStr,
    {
        let read = fs::read_to_string(&self.file).unwrap();
        T::from_str(&read)
    }

    pub fn get_csv(&self) -> Result<Vec<String>> {
        self.get_split(',')
    }

    pub fn get_csv_as<T>(&self) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.get_split_as(',')
    }

    pub fn get_lines(&self) -> Result<Vec<String>> {
        Ok(fs::read_to_string(&self.file)?
            .lines()
            .map(|l| l.to_owned())
            .collect())
    }

    pub fn get_lines_as<T>(&self) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.lines().filter_map(|x| x.parse().ok()).collect())
    }

    pub fn get_grouped_as<T>(&self) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.get_split_str_as("\n\n")
    }

    pub fn get_split(&self, pat: char) -> Result<Vec<String>> {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.split(pat).map(|x| x.to_owned()).collect())
    }

    pub fn get_split_str(&self, pat: &str) -> Result<Vec<String>> {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.split(pat).map(|x| x.to_owned()).collect())
    }

    pub fn get_split_as<T>(&self, pat: char) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.split(pat).filter_map(|x| x.parse().ok()).collect())
    }

    pub fn get_split_str_as<T>(&self, pat: &str) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.split(pat).filter_map(|x| x.parse().ok()).collect())
    }
}
