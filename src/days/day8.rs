use std::{collections::{HashMap, HashSet}, fmt};
use colored::{ColoredString, Colorize};
use rand::Rng;
use crate::days::solver::AdventSolver;

#[derive(Debug)]
enum FieldObject {
    Antennae(char, bool),
    Antinode,
    Empty
}

struct AntennaeField {
    grid: Vec<Vec<FieldObject>>,
    locs: HashMap<char, Vec<(usize, usize)>>
}

struct ConstructionCrew;


impl ConstructionCrew {
    fn measure_distance(node_a: (usize, usize), node_b: (usize, usize)) -> usize {
        let x = node_a.0.abs_diff(node_b.0);
        let y = node_a.1.abs_diff(node_b.1);

        x + y
    }
}

impl fmt::Display for AntennaeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut custom_colors: HashMap<char, [u8; 3]> = HashMap::new();
        let mut rng = rand::thread_rng();
        let _ = self.grid.iter().fold(String::new(), |mut rs, row| {
            let row_mapped = row.iter().fold(String::new(), |mut s, fo| {
                let fos: ColoredString = match fo {
                    FieldObject::Antennae(c, has_antinode) => {
                        let color = match custom_colors.get(c) {
                            Some(color) => *color,
                            None => {

                                let color = [rng.gen_range(55..200), rng.gen_range(55..200), rng.gen_range(0..200)];
                                custom_colors.insert(*c, color);
                                color
                            }
                        };
                        if *has_antinode {
                            c.to_string().white()
                        } else {
                            c.to_string().truecolor(color[0], color[1], color[2])
                        }
                    },
                    FieldObject::Antinode => "#".to_string().yellow(),
                    _ => ".".to_string().green()
                };
                write!(f, "{}", fos).unwrap();
                s.push_str(&fos);
                s
            });

            rs.push_str(&row_mapped);
            write!(f, "\n").unwrap();
            rs
        });
        write!(f, "\n").unwrap();
        write!(f, "{:?}", self.locs)
    }
}



impl AntennaeField {
    fn get_possible_antinode_locs(&self, first: (usize, usize), second: (usize, usize)) -> Vec<(isize, isize)> {
        let max = self.grid.len() as isize;
        let min = 0;
        let diff = self.get_distance_diff(first, &second);

        let mut locs = vec![];
        let mut loc = (first.0 as isize + diff.0, first.1 as isize + diff.1);

        while loc.0 >= min && loc.0 < max && loc.1 >= min && loc.1 < max {
            locs.push(loc);
            loc = (loc.0 + diff.0, loc.1 + diff.1);
        }

        loc = (second.0 as isize + -diff.0, second.1 as isize + -diff.1);

        while loc.0 >= min && loc.0 < max && loc.1 >= min && loc.1 < max {
            locs.push(loc);
            loc = (loc.0 + -diff.0, loc.1 + -diff.1);
        }

        locs
    }

    fn get_pair_distance(&self, first: (usize, usize), second: &(usize, usize)) -> [(isize, isize); 2] {
        let diff = self.get_distance_diff(first, second);

        [
            (
                first.0 as isize + diff.0,
                first.1 as isize + diff.1
            ),
            (
                second.0 as isize + -diff.0,
                second.1 as isize + -diff.1
            )
        ]
    }

    fn get_distance_diff(&self, first: (usize, usize), second: &(usize, usize)) -> (isize, isize) {
        (
            first.0 as isize - second.0 as isize,
            first.1 as isize - second.1 as isize
        )
    }

    fn place_antinodes(&mut self) {
        for c in self.locs.keys() {
            let freq_nodes = self.locs.get(c).unwrap();
            let size = self.grid.len() as isize;
            let mut loc = freq_nodes[0];
            let mut idx = 1;

            while idx < freq_nodes.len() {
                for node in freq_nodes[idx..].iter() {
                    let distance = self.get_pair_distance(loc, node);
                    for coord in distance {

                        if coord.0 >= 0 && coord.0 < size && coord.1 >= 0 && coord.1 < size {
                            let el = self.grid.get_mut(coord.0 as usize).unwrap().get_mut(coord.1 as usize).unwrap();
                            *el = match el {
                                FieldObject::Antennae(c, _) => FieldObject::Antennae(*c, true),
                                _ => FieldObject::Antinode
                            }
                        }
                    }
                }

                loc = freq_nodes[idx];
                idx += 1;
            }
        }
    }

    fn place_multi_antinodes(&mut self) {
        for c in self.locs.keys() {
            let freq_nodes = self.locs.get(c).unwrap();
            let size = self.grid.len() as isize;
            let mut loc = freq_nodes[0];
            let mut idx = 1;
            let mut appeared = 0;
            let mut distances: HashSet<(isize, isize)> = HashSet::new();
            while idx < freq_nodes.len() {
                for node in freq_nodes[idx..].iter() {
                    let diff = self.get_distance_diff(loc, node);
                    let distance = self.get_possible_antinode_locs(loc, *node);
                    for coord in distance {
                        let el = self.grid.get_mut(coord.0 as usize).unwrap().get_mut(coord.1 as usize).unwrap();
                        *el = match el {
                            FieldObject::Antennae(c, _) => FieldObject::Antennae(*c, true),
                            _ => FieldObject::Antinode
                        }
                    }
                    println!("{}", self);
                }

                loc = freq_nodes[idx];
                idx += 1;
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        let r: usize = self.grid.iter().fold(0, |acc, row| {
            row.iter().fold(0, |a, c| {
                match c {
                    FieldObject::Antennae(_, true) => a + 1,
                    FieldObject::Antinode => a + 1,
                    _ => a
                }
            }) + acc
        });
        r
    }
}

trait Day8 {
    fn parse(&self) -> AntennaeField;
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
}

impl Day8 for AdventSolver {
    fn parse(&self) -> AntennaeField {
        let mut ant_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let grid: Vec<Vec<FieldObject>> = self.input
            .lines().enumerate()
            .map(|(i, f)| {
                f.chars().enumerate().map(|(ri, c)| {
                    match c {
                        '.' => FieldObject::Empty,
                        '#' => FieldObject::Antinode,
                        _ => {
                            match ant_locs.get_mut(&c) {
                                Some(locs) => {
                                    locs.push((i, ri));
                                },
                                None => {
                                    ant_locs.insert(c, vec![(i, ri)]);
                                }
                            };
                            FieldObject::Antennae(c, false)
                        }
                    }
                }).collect()
            }).collect();


        AntennaeField {
            grid,
            locs: ant_locs,
        }
    }

    fn part_one(&self) -> usize {
        let mut field = self.parse();
        field.place_antinodes();
        println!("{} : ", field);
        field.count_antinodes()
    }

    fn part_two(&self) -> usize {
        let mut field = self.parse();
        field.place_multi_antinodes();
        println!("{} : ", field);
        field.count_antinodes()
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
    println!("red: {}", "am I tho".green());
}

