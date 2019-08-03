#[macro_use]
extern crate mathion;
extern crate time;

use mathion::prelude::*;
use time::PreciseTime;
use std::f64::consts::PI;
use std::f64::consts::E;


fn main() {
    let x: Symbol = symbol!("x");
    let f_x = function!(log(function!(x), E, -1.0));
    let start = PreciseTime::now();
    let result = romberg(f_x, "x", 20, (0.0, 229.0)).unwrap();
    let end = PreciseTime::now();
    println!("Took {} seconds. result is {}.", start.to(end), result);
}