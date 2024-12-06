use super::solver::{AdventSolver, DaySolver};



impl DaySolver for AdventSolver {
    fn part_one(&self) -> usize {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for line in self.input.split("\n") {
            let raw_nums: Vec<i32> = line.split_whitespace().map(|c| {
                c.parse::<i32>().unwrap()
            }).collect();
            left.push(raw_nums[0]);
            right.push(raw_nums[1]);
        }

        left.sort();
        right.sort();

        let distance: i32 = left.into_iter().zip(right.into_iter()).map(|(l,r)| {
            i32::abs(l - r)
        }).rev().sum();

        distance as usize
    }


    fn part_two(&self) -> usize {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for line in self.input.split("\n") {
            let raw_nums: Vec<i32> = line.split_whitespace().map(|c| {
                c.parse::<i32>().unwrap()
            }).collect();
            left.push(raw_nums[0]);
            right.push(raw_nums[1]);
        }
        let mut similarity: usize = 0;
        for left_num in left {
            let count = right.iter().filter(|c| **c == left_num).count();

            similarity += (left_num * count as i32) as usize;
        }

        similarity as usize
    }
}

pub fn solve(fpath: &str, part: &str) {
    let solver = AdventSolver::new(fpath);
    let result = if part == "1" {
        solver.part_one()
    } else {
        solver.part_two()
    };

    println!("result: {:?}", result);
}