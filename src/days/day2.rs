use super::solver::AdventSolver;
// this was lousy; first part was straightforward, but I really overcomplicated / misunderstood the

trait Day2 {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
    fn is_safe(&self, nums: Vec<LevelSegment>) -> bool;
    fn state(&self, x: &i32, y: &i32) -> i32;
}

#[derive(Debug, PartialEq)]
enum LevelStatus {
    Bad,
    Good
}

#[derive(Debug)]
struct LevelSegment {
    increasing: bool,
    status: LevelStatus,
    vals: (i32, i32)
}


impl LevelSegment {
    fn new(l: i32, r: i32) -> Self {
        let diff = i32::abs(l - r);

        let mut status = LevelStatus::Good;

        if diff > 3 || diff == 0 {
            status = LevelStatus::Bad;
        }

        Self {
            increasing: l < r,
            status,
            vals: (l, r)
        }
    }
}

impl Day2 for AdventSolver {
    fn is_safe(&self, nums: Vec<LevelSegment>) -> bool {
        let increasing = nums.get(0).unwrap().increasing;
        nums
            .iter()
            .all(|segment| {
                segment.status == LevelStatus::Good && segment.increasing == increasing
            })
    }

    fn part_one(&self) -> usize {
        let mut digits: Vec<Vec<LevelSegment>> = vec![];
        for line in self.input.split("\n") {
            let raw_nums: Vec<i32> = line.split_whitespace().map(|c| {
                c.parse::<i32>().unwrap()
            }).collect();
            
            let segments: Vec<LevelSegment> = raw_nums.windows(2).map(|x| {
                LevelSegment::new(*x.get(0).unwrap(), *x.get(1).unwrap())
            }).collect();
            digits.push(segments);

        }
        let mut result= 0;

        for digit_set in digits {
            if self.is_safe(digit_set) {
                result += 1;
            }
        }
        
        result as usize
    }


    fn part_two(&self) -> usize {
        let mut digits: Vec<Vec<i32>> = vec![];
        for line in self.input.split("\n") {
            let raw_nums: Vec<i32> = line.split_whitespace().map(|c| {
                c.parse::<i32>().unwrap()
            }).collect();
            digits.push(raw_nums);
        }
        
        let mut result = 0;
        
        for nums in digits {
            if nums.last().unwrap() < nums.first().unwrap() {

            }
        }

        result
    }

    fn state(&self, x: &i32, y: &i32) -> i32 {
        if x < y {
            1
        } else if x > y {
            -1
        } else {
            0
        }
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