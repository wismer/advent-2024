use core::{unimplemented};
use std::fs::File;
use std::io::prelude::*;

use crate::days::{
    day1
};

use super::{day2, day3, day4, day5, day6};

pub fn solver(fpath: &str, day: &str, part: &str) {
    match day {
        "1" => day1::solve(fpath, part),
        "2" => day2::solve(fpath, part),
        "3" => day3::solve(fpath, part),
        "4" => day4::solve(fpath, part),
        "5" => day5::solve(fpath, part),
        "6" => day6::solve(fpath, part),
        _ => unimplemented!()
    }
}


pub struct AdventSolver {
    pub input: String
}

pub trait DaySolver {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
}

impl AdventSolver {
    pub fn new(fpath: &str) -> Self {
        let mut buf: String = String::new();
        let mut file = File::open(fpath).unwrap();
        file.read_to_string(&mut buf).unwrap();

        AdventSolver {
            input: buf
        }
    }
}