#[macro_use]
extern crate mathion;
extern crate time;

use mathion::prelude::*;
use time::PreciseTime;

fn main() {
    let x: Symbol = symbol!("x");
    let start = PreciseTime::now();
    let end = PreciseTime::now();
    let result = function!(x + 1.0) * function!(x.powf(2.0) + 1.0);
    println!("Took {} seconds. result is {}.", start.to(end), result);
}