use crate::days::solver::AdventSolver;

#[derive(Debug)]
struct Equation {
    result: usize,
    numbers: Vec<usize>
}

// I did not write this, adam chalmers did. Thanks/Sorry adam! I don't understand math!
fn is_solvable(goal: usize, items: Vec<usize>, allow_concat: bool) -> bool {
    // Base case.
    let Some((curr, rest)) = items.split_last() else {
        return goal == 0;
    };

    // Three recursive cases:
    // 1. Using + operation
    let diff = if goal > *curr {
        goal - curr
    } else {
        curr - goal
    };
    println!("curr {}, goal {}", curr, goal);
    is_solvable(diff, rest.to_vec(), allow_concat)
    // 2. Using * operation
    || goal % curr == 0 && is_solvable(goal / curr, rest.to_vec(), allow_concat)
    // 3. Using || operation
    || {
        let new_goal = diff;
        let tens = 10usize.pow(curr.ilog10() + 1);
        allow_concat && new_goal % tens == 0 && is_solvable(new_goal / tens, rest.to_vec(), allow_concat)
    }
}


impl Equation {
    fn operations(&self, val: usize, idx: usize) -> bool {
        match self.numbers.get(idx) {
            Some(n) => {
                self.operations(val + n, idx + 1) || self.operations(val * n, idx + 1)
            },
            None => {
                if val == self.result {
                    true
                } else {
                    self.operations_with_concatenate()
                }
            }
        }
    }

    fn operations_with_concatenate(&self, ) -> bool {
        false
    }

    fn concatenate(&self, x: usize, y: usize) -> usize {
        let mut r = x.to_string();
        r.push_str(y.to_string().as_str());
        r.parse::<usize>().unwrap()
    }
}

trait Day7 {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
    fn parse(&self) -> Vec<Equation>;
}

impl Day7 for AdventSolver {
    fn parse(&self) -> Vec<Equation> {
        let equations: Vec<Equation> = self.input
        .lines()
        .map(|raw_eq| {
            let r: Vec<&str> = raw_eq.split(": ").collect();
            let result = r.get(0).unwrap().parse::<usize>().unwrap();
            let rest = r.get(1).unwrap().split(" ").map(|n| {
                n.parse::<usize>().unwrap()
            }).collect();

            Equation {
                numbers: rest,
                result
            }
        }).collect();

        equations
    }
    fn part_one(&self) -> usize {
        let data = self.parse();
        let mut result = 0;
        for equation in data {
            if equation.operations(equation.numbers[0], 1) {
                result += equation.result;
            }
        }

        result
    }

    fn part_two(&self) -> usize {
        let data = self.parse();
        let mut result = 0;
        for equation in data {
            if is_solvable(equation.result, equation.numbers, true) {
                result += equation.result;
            }
        }

        result
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



#[test]
fn test_part_one() {
    let solver = AdventSolver::new("/Users/matt/personal/advent-2024/day7.txt");
    let result = solver.part_one();

    assert_eq!(result, 3749);
}

#[test]
fn test_concatenate() {
    let eq = Equation {
        numbers: vec![],
        result: 0
    };

    let result = eq.concatenate(12, 125);

    assert_eq!(result, 12125);
}