use std::{collections::HashSet, thread::yield_now};

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
    fn check_coord(&self, pos: (usize, usize)) -> Option<&PlotPoint> {
        self.map.get(pos.0).and_then(|x| x.get(pos.1))
    }

    fn get_next_coordinate(&self) -> Option<(usize, usize)> {
        let max = (self.map.len() - 1) as isize;
        let (current_position, current_direction) = &self.guard_location;
        let coord = match current_direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        };
        let x = current_position.0 as isize + coord.0;
        let y = current_position.1 as isize + coord.1;

        if x < 0 || x > max || y < 0 || y > max {
            return None
        }

        Some((x as usize, y as usize))
    }

    fn should_rotate(&self, coordinate: (usize, usize)) -> bool {
        match self.check_coord(coordinate) {
            None => false,
            Some(pp) => {
                match pp {
                    PlotPoint::Structure => {
                        true
                    },
                    _ => false
                }
            }
        }
    }

    fn update_position(&mut self, pos: (usize, usize)) {
        self.guard_location = (Point(pos.0, pos.1), self.guard_location.1);
    }

    fn rotate(&mut self) {
        let new_direction = match self.guard_location.1 {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down
        };

        self.guard_location = (Point(self.guard_location.0.0, self.guard_location.0.1), new_direction);
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
        let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut map = self.parse();

        visited_positions.insert((map.guard_location.0.0, map.guard_location.0.1));
        // evaluate direction (up, down, etc)
        // modify current position with direction context
        // with modified position, check the map for validitity
        // if it's a structure, update direction & rerun step 1 & 2

        while let Some(n) = map.get_next_coordinate() {
            if map.should_rotate(n) {
                // rotate but don't move from {:, coordinate?}
                map.rotate();
            } else {
                map.update_position(n);
                visited_positions.insert(n);
            }
        }
        visited_positions.len()
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

