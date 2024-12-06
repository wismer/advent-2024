use std::env::args;
use self::days::solver::solver;

pub mod days;


fn main() {
    println!("Hello, world!");
    let inputpath = args().nth(1).unwrap();
    let samplepath = args().nth(2).unwrap();
    let part = args().nth(3).unwrap();
    solver(inputpath.as_ref(), samplepath.as_ref(), &part);
}
