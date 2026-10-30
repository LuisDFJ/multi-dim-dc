mod lattice;
mod utils;
mod cell;
mod set;

use self::utils::*;
use self::lattice::*;
use self::cell::*;
use self::set::*;

type Mask = Vec<bool>;

#[derive(Debug,Clone)]
pub struct Set(Vec<u8>);


pub struct Lattice {
    n : u8,
    u : Set,
}

#[derive(Debug)]
pub struct Cell {
    n : u8,
    k : u8,
    m : u8,
    mask : Mask,
    offset : Set,
    face : Set,
}
