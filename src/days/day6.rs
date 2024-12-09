use std::collections::HashSet;

use crate::days::solver::AdventSolver;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq)]
enum PlotPoint {
    Structure,
    Free,
    Guard(Direction)
}

#[derive(Debug)]
struct GuardMap {
    map: Vec<Vec<PlotPoint>>,
    guard_location: (Point, Direction)
}


trait Day6 {
    fn parse(&self) -> GuardMap;
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
}

impl GuardMap {
    fn get_next_coord(&mut self) -> Option<(usize, usize)> {
        let max = self.map.len();

        let (current_pos, direction) = &self.guard_location;
        println!("DID I UPDATE: {:?}", self.guard_location);
        match direction {
            Direction::Down => {
                if current_pos.0 == max - 1 {
                    None
                } else {
                    Some((current_pos.0 + 1, current_pos.1))
                }
            },
            Direction::Up => {
                if current_pos.0.checked_sub(1).is_none() {
                    None
                } else {
                    Some((current_pos.0 - 1, current_pos.1))
                }
            },
            Direction::Right => {
                if current_pos.1 == max - 1 {
                    None
                } else {
                    Some((current_pos.0, current_pos.1 + 1))
                }
            },
            Direction::Left => {
                if current_pos.1 == 0 {
                    None
                } else {
                    Some((current_pos.0, current_pos.1 - 1))
                }
            },
            _ => None
        }
    }

    fn update_position(&mut self, pos: (usize, usize)) {
        println!("update pos : {:?}", pos);
        match self.map.get(pos.0).and_then(|row| row.get(pos.1)) {
            Some(pp) => {
                match pp {
                    PlotPoint::Structure => {
                        let new_direction = match self.guard_location.1 {
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down
                        };
                        self.guard_location.1 = new_direction;
                    },
                    _ => {
                        self.guard_location = (Point(pos.0, pos.1), self.guard_location.1);
                    }
                }
            },
            None => {}
        }
    }
}

impl Day6 for AdventSolver {
    fn parse(&self) -> GuardMap {
        let mut guard_pos = (Point(0, 0), Direction::Down);
        let map: Vec<Vec<PlotPoint>> = self.input.split("\n").enumerate().map(|(row, line)| {
            line.chars().enumerate().map(|(col, c)| {
                match c {
                    '.' => PlotPoint::Free,
                    '#' => PlotPoint::Structure,
                    _ => {
                        let direction = match c {
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            'v' => Direction::Down,
                            '^' => Direction::Up,
                            _ => unreachable!()
                        };
                        guard_pos = (Point(row, col), direction);
                        PlotPoint::Guard(direction)
                    }
                }
            }).collect::<Vec<PlotPoint>>()
        }).collect();
        
        GuardMap {
            map,
            guard_location: guard_pos
        }
    }

    fn part_one(&self) -> usize {
        let mut result = 0;
        let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut map = self.parse();

        visited_positions.insert((map.guard_location.0.0, map.guard_location.0.1));
        while let Some(n) = map.get_next_coord() {
            println!("n: {:?}", n);
            visited_positions.insert(n);
            map.update_position(n);
            println!("after: {:?}", map.guard_location);
        }
        println!("map: {:?}", visited_positions.len());
        result
    }

    fn part_two(&self) -> usize {
        let mut result = 0;
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

