#[macro_use]
extern crate mathion;
extern crate time;

use mathion::prelude::*;
use time::PreciseTime; 
use std::any::Any;


fn main() {
    let A: Matrix = matrix![row![5.0, 1.0, 0.0], 
                            row![9.0, -4.0, 0.0],
                            row![16.0, -11.0, -7.0]];
    let B: Matrix = matrix![row![8.0], 
                            row![6.0], 
                            row![5.0]];
    let start = PreciseTime::now();
    let result = solve_by_lu(A, B);
    let end = PreciseTime::now();
    println!("Took {} seconds. result is {}.", start.to(end), result);
}