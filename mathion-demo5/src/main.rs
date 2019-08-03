#[macro_use]
extern crate mathion;
extern crate time;

use mathion::prelude::*;
use time::PreciseTime;
use std::f64::consts::PI;
use std::f64::consts::E;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let x: Symbol = symbol!("x");
    let y: Symbol = symbol!("y");
    let alpha = 1.0;
    let beta = 0.01;
    let delta = -1.0;
    let gamma = 0.02;
    let f_1 = function!(alpha * x - beta * x * y);
    let f_2 = function!(gamma * x * y + delta * y);
    let vars = var![("x", 20.0), ("y", 20.0)];
    let start = PreciseTime::now();
    let result = solve_ode4(vec![f_1, f_2], "t", vars, (0.0, 30.0), 10000).unwrap();
    let end = PreciseTime::now();
    println!("Took {} seconds.", start.to(end));
    let mut a = File::create("a.dat").unwrap();
    let mut b = File::create("b.dat").unwrap();
    for i in 0..result.len() {
        a.write_all(format!("{} {}\n", i as f64 * 0.003, result[i].nth(0).1).as_bytes());
        b.write_all(format!("{} {}\n", i as f64 * 0.003, result[i].nth(1).1).as_bytes());
    }
}