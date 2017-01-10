#[macro_use]
extern crate arrayref;
extern crate num;

mod castutils;
mod solver;

use solver::CellType;

fn main() {
    println!("{:?}", CellType::Peg.solve_sides(&[Some(false), None, Some(false), Some(false)]));
}
