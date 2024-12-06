use crate::days::solver::AdventSolver;


type Chars = Vec<Vec<char>>;
struct Point(isize, isize);

impl Point {
    fn next(&self, direction: (isize, isize), limit: isize) -> Option<[(isize, isize); 3]> {
        let r = [
            (self.0 + (direction.0 * 1), self.1 + (direction.1 * 1)),
            (self.0 + (direction.0 * 2), self.1 + (direction.1 * 2)),
            (self.0 + (direction.0 * 3), self.1 + (direction.1 * 3))
        ];

        if r.iter().any(|n| n.0 < 0 || n.1 < 0 || n.0 >= limit || n.1 >= limit) {
            None
        } else {
            Some(r)
        }
    }

    fn next_x(&self, limit: isize) -> Option<[Point; 4]> {
        let r = [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0 + 1, self.1 + 1),
            Point(self.0 + 1, self.1 - 1)
        ];

        if r.iter().any(|n| n.0 < 0 || n.1 < 0 || n.0 >= limit || n.1 >= limit) {
            None
        } else {
            Some(r)
        }
    }

    fn find(&self, direction: (isize, isize), chars: &Vec<Vec<char>>) -> bool {
        let mut idx = 0;
        let xmas = ['M', 'A', 'S'];
        match self.next(direction, chars.len() as isize) {
            Some(coords) => {
                for c in coords {
                    let target = xmas[idx];
                    let current_char = chars[c.0 as usize][c.1 as usize];
                    println!("current char: {}", current_char);
                    if current_char  == target {
                        idx += 1;
                    } else {
                        return false
                    }
                    // match chars.get(c.0 as usize) {
                    //     Some(row) => row.get(c.1 as usize)
                    // }
                }
                true
            },
            None => false
        }
    }
}
trait Day4 {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
    fn check(&self, origin: Point, chars: &Vec<Vec<char>>) -> Vec<char>;
}

impl Day4 for AdventSolver {
    fn check(&self, origin: Point, chars: &Vec<Vec<char>>) -> Vec<char> {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1)
        ];





        let target = ['M', 'A', 'S'];
        let mut pts: Vec<char> = vec![];

        pts
    }

    fn part_one(&self) -> usize {
        let char_rep: Vec<Vec<char>> = self.input.split("\n")
            .map(|x| x.chars().collect())
            .collect();

        let (mut x, mut y) = (0usize, 0usize);
        let mut result = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1)
        ];
        for x in 0..char_rep.len() {
            for y in 0..(char_rep[x].len()) {
                let pt = Point(x as isize, y as isize);
                for direction in directions {
                    if char_rep[x][y] == 'X' && pt.find(direction, &char_rep) {
                        result += 1;
                    }
                }
            }
        }

        result
    }

    fn part_two(&self) -> usize {
        let char_rep: Vec<Vec<char>> = self.input.split("\n")
            .map(|x| x.chars().collect())
            .collect();

        let mut result = 0;

        for x in 0..char_rep.len() {
            for y in 0..(char_rep[x].len()) {
                let pt = Point(x as isize, y as isize);
                
                if char_rep[x][y] == 'A' {
                    let x_map = pt.next_x(char_rep.len() as isize);
                    if x_map.is_some() {
                        let xcoords = x_map.unwrap();
                        let xcontents = xcoords.map(|pt| char_rep[pt.0 as usize][pt.1 as usize]);

                        match xcontents {
                            ['M', 'M', 'S', 'S'] | ['S', 'S', 'M', 'M'] | ['S', 'M', 'M', 'S'] | ['M', 'S', 'S', 'M'] => result += 1,
                            // ['M', 'S', 'M', 'S'] | ['S', 'M', 'S', 'M'] => {},
                            _ => {}
                        }
                        // let ugh = [xcontents[0], xcontents[2]];
                        // let groan = [xcontents[1], xcontents[3]].sort();
                        // if ugh[0] == 'M' && ugh[1] == 'S' && groan[0] == 'M' && groan[1] == 'S' {
                        //     result += 1;
                        // }
                    }
                }
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

