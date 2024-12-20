use std::{collections::{HashMap, HashSet}, thread::yield_now};

use crate::days::solver::AdventSolver;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
        let coord = self.get_direction_modifier(current_direction);
        let x = current_position.0 as isize + coord.0;
        let y = current_position.1 as isize + coord.1;

        if x < 0 || x > max || y < 0 || y > max {
            return None
        }

        Some((x as usize, y as usize))
    }

    fn get_direction_modifier(&self, direction: &Direction) -> (isize, isize) {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
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
        let new_direction = self.next_direction();
        self.guard_location = (Point(self.guard_location.0.0, self.guard_location.0.1), new_direction);
    }

    fn next_direction(&self) -> Direction {
        match self.guard_location.1 {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down
        }
    }

    fn set_structure(&mut self, x: usize, y: usize) {
        let cell = self.map.get_mut(x).and_then(|pp| pp.get_mut(y)).unwrap();
        *cell = PlotPoint::Structure;
    }

    fn reset(&mut self, original_map: Vec<Vec<PlotPoint>>, original_start: (Point, Direction)) {
        self.map = original_map;
        self.guard_location = original_start;
    }

    fn clear_structure(&mut self, x: usize, y: usize) {
        let pp = self.map.get_mut(x).and_then(|r| r.get_mut(y)).unwrap();
        *pp = PlotPoint::Free;
    }

    fn structure_in_path(&self, origin: (usize, usize), direction: &Direction) -> bool {
        let modifier = self.get_direction_modifier(direction);
        let mut coord = (origin.0 + modifier.0 as usize, origin.1 + modifier.1 as usize);
        let mut has_structure = false;
        while let Some(pp) = self.check_coord(coord) {
            match pp {
                PlotPoint::Structure => {
                    has_structure = true;
                    break;
                },
                _ => {
                    coord = (coord.0 + modifier.0 as usize, coord.1 + modifier.1 as usize);
                }
            }
        }

        has_structure
    }


    /*
        What I Want To Do Here

        this is supposed to be like a subset of part one behavior
        it's supposed to traverse the 2d grid as normal BUT
        there's another loop that needs to happen:

        BEFORE each time the guard advances to the next cell
        use a temporary Direction and with that, check all the cells that the guard WOULD
        pass through. Return a bool on whether there is a structure in the path. 

        
        fn check_path(origin, direction) -> bool - from the origin + direction, check if path has a structure

        IF THERE IS
            insert the structure because it might get encountered again
            insert the guards INCOMING location and Direction into HashSet
            rotate the guard
            advance the guard until...
                at structure, insert the guards INCOMING location and Direction into HashSet
                rotate the guard, etc

            IF the guard has already encountered the structure before (same direction, same point in grid)
                uptick the encounter COUNT?
                IF this is the SECOND time it's been encountered, then it means we're in a loop 

            continue until off the map?
            reset the grid, but preserve the "origin point" and direction
        IF THERE ISNT
            continue normal behavior?
                check next point
                



        a structure is placed there instead

        
    */ 
    fn contains_loop(&mut self) -> bool {
        // turn locations
        let mut turn_locations: HashMap<(usize, usize), usize> = HashMap::new();
        // set structure
        println!("blergh", );
        let next_pt = self.get_next_coordinate().unwrap();
        self.set_structure(next_pt.0, next_pt.1);
        self.rotate();

        while let Some(c) = self.get_next_coordinate() {
            if self.should_rotate(c) {
                match turn_locations.get_mut(&c) {
                    Some(n) => *n += 1,
                    None => {
                        turn_locations.insert(c, 1);
                    }
                }

                self.rotate();
            } else {
                self.update_position(c);
            }
        }

        self.clear_structure(next_pt.0, next_pt.1);

        println!("{:?}", turn_locations);
        turn_locations.len() > 0
        // treat it like a structure

        // first condition: Will the new direction end in another structure?
        // and will it "touch" each turning point at least twice?
        // record each turn, and increment the value by one whenever it has been visited.
        // the moment a point has been visited a third time?
        // remove all other entries that have visit counts of less than 2
        // pattern found?


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
        // get data setup first
        let mut guard_map = self.parse();
        while !guard_map.contains_loop() {
            // check 
        }

        // this is a hard one
        // This what I am thinking...
        // anytime the guard moves, check one spot ahead in the direction it is heading in
        // 1. suppose it hits a structure
        // 2. and in the direction it WOULD go in, will it end up hitting another structure
        // 3. and after that, will it end up hitting ANOTHER structure (and so on)
        // the failure case here is the guard moves off the map
        // the success case here is harder to figure out.
        // I guess it would be if it made a successful loop? 
        // How is that measured or tracked?
        // use a HashSet to keep track of its path, and see if there is a collision? 
        // That won't work... I don't think.
        // also, each time I "place" a structure, and successfully find a loop,
        // I should reset the grid.
        // and how many attempts is too many? The original path is exhausted?
        // so, to recap, this is how I Should Write It
        // establish what the loop will be
        // function for identifying a loop
        // function for placing a structure
        // function for reseting map?

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

