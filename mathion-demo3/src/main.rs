#[macro_use]
extern crate mathion;
extern crate time;

use mathion::prelude::*;
use time::PreciseTime;
use std::f64::consts::FRAC_2_SQRT_PI;


fn main() {
    let t: Symbol = symbol!("t");
    let f_x = function!(exp(-function!(t.powf(2.0)), 1.0));
    let start = PreciseTime::now();
    let result = FRAC_2_SQRT_PI * romberg(f_x, "t", 15, (0.0, 100.0)).unwrap();
    let end = PreciseTime::now();
    println!("Took {} seconds. result is {}.", start.to(end), result);
}