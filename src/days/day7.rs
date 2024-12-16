use crate::days::solver::AdventSolver;

#[derive(Debug)]
struct Equation {
    result: usize,
    numbers: Vec<usize>
}

enum Op {
    Mult,
    Add
}

impl Equation {
    fn operations(&self, val: usize, idx: usize) -> bool {
        match self.numbers.get(idx) {
            Some(n) => {
                self.operations(val + n, idx + 1) || self.operations(val * n, idx + 1)
            },
            None => {
                val == self.result
            }
        }
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
        0
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