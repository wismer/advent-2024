use regex::Regex;

use crate::days::solver::AdventSolver;

trait Day3 {
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;
    fn calculate_chunk(&self, text: &str) -> usize;
}

impl Day3 for AdventSolver {
    fn part_one(&self) -> usize {
        self.calculate_chunk(&self.input)
    }

    fn calculate_chunk(&self, text: &str) -> usize {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut total = 0;
        for r in re.captures_iter(text) {
            let first_num = r.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let second_num = r.get(2).unwrap().as_str().parse::<usize>().unwrap();


            total += first_num * second_num;
        }
        // let r: Vec<(usize, &str)> = self.input.match_indices("mul(").collect();


        total
    }

    fn part_two(&self) -> usize {
        let dont_markers: Vec<(usize, &str)> = self.input.match_indices("don't()").collect();
        let do_markers: Vec<(usize, &str)> = self.input.match_indices("do()").collect();
        let mut result = 0;
        let mut dont_marker_idx = 0;
        let mut do_marker_idx = 0;
        let mut valid_start = 0;
        
        // since the text is valid in the beginning, we mark the end as the 
        // first idx of the `dont_markers`
        let mut valid_end = dont_markers[dont_marker_idx].0;

        loop {
            // take that chunk and parse it for the value
            result += self.calculate_chunk(&self.input[valid_start..valid_end]);
            // next, advance the valid start to where the do_marker match index is greater 
            // than the current dont_marker match index
            while do_markers[do_marker_idx].0 < valid_end {
                do_marker_idx += 1;
            }
            valid_start = do_markers[do_marker_idx].0;
            // the start idx is now greater than the current match index for a "dont" block
            // now advance the match index for dont_markers until the match index is 
            // greater than the current start_index


            while dont_markers.get(dont_marker_idx).is_some() && dont_markers[dont_marker_idx].0 < valid_start {
                dont_marker_idx += 1;
            }

            // save the valid_end

            if dont_marker_idx >= dont_markers.len() {
                // last don't block. Use the length of the string as the end
                result += self.calculate_chunk(&self.input[valid_start..(self.input.len())]);
                break
            }

            valid_end = dont_markers[dont_marker_idx].0;

            // now think about how to end the loop

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