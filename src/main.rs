use aoc25::run_w_args;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    println!("{}", run_w_args(&args));
}
