use super::solver::AdventSolver;

#[derive(Debug)]
struct PageRule(usize, usize);
#[derive(Debug)]
struct PageUpdate {
    pub updates: Vec<usize>
}




trait Day5 {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;

    fn parse(&self) -> (Vec<PageRule>, Vec<PageUpdate>);
}


#[derive(PartialEq)]
enum Validation {
    Ignore,
    Valid,
    Invalid(usize, usize)
}

impl PageUpdate {
    fn validate(&self, rule: &PageRule) -> Validation {
        let updates = &self.updates;
        let lpos = updates.iter().rposition(|x| *x == rule.0);
        let rpos = updates.iter().rposition(|x| *x == rule.1);

        match (lpos, rpos) {
            (Some(l), Some(r)) => {
                if l < r {
                    Validation::Valid
                } else {
                    Validation::Invalid(l, r)
                }
            },
            _ => Validation::Ignore
        }
    }

    fn rearrange(&mut self, rules: &Vec<PageRule>) -> usize {
        let midpoint = self.updates.len() / 2;

        loop {
            for rule in rules {
                match self.validate(rule) {
                    Validation::Invalid(lpos, rpos) => self.updates.swap(lpos, rpos),
                    _ => {}
                }
            }

            if !rules.iter().any(|r| {
                match self.validate(r) {
                    Validation::Invalid(_l, _r) => true,
                    _ => false
                }
            }) {
                break
            }
        }
        for rule in rules {
            match self.validate(rule) {
                Validation::Invalid(lpos, rpos) => self.updates.swap(lpos, rpos),
                _ => {}
            }
        }

        self.updates[midpoint]
    }
}

impl Day5 for AdventSolver {
    fn parse(&self) -> (Vec<PageRule>, Vec<PageUpdate>) {
        // split the sections apart by two successive newlines
        let raw_sections: Vec<&str> = self.input.split("\n\n").collect();

        (
            raw_sections[0].split("\n").map(|order| {
                let o: Vec<usize> = order.split("|").map(|c| {
                    c.parse::<usize>().unwrap()
                }).collect();

                PageRule(o[0], o[1])
            }).collect::<Vec<PageRule>>(),
            raw_sections[1].split("\n").map(|raw_update| {
                PageUpdate {
                    updates: raw_update.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                }
            }).collect::<Vec<PageUpdate>>()
        )
    }

    fn part_one(&self) -> usize {
        let (rules, updates) = self.parse();
        let mut result = 0;
        for update in updates {
            let is_invalid = rules.iter().any(|r| {
                match update.validate(r) {
                    Validation::Invalid(_lpos, _rpos) => true,
                    _ => false
                }
            });

            if !is_invalid {
                let mid = update.updates.len() / 2;
                result += update.updates[mid];
            }
        }

        result
    }

    fn part_two(&self) -> usize {
        let (rules, mut updates) = self.parse();
        let mut result = 0;
        for update in updates.iter_mut() {
            let invalid_rules: Vec<&PageRule> = rules.iter().filter(|r| {
                match update.validate(r) {
                    Validation::Invalid(_, _) => true,
                    _ => false
                }
            }).collect();

            if invalid_rules.len() > 0 {
                result += update.rearrange(&rules);                
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

