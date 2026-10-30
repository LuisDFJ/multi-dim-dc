mod lattice;
mod utils;
mod cell;

use self::utils::*;
use self::lattice::*;
use self::cell::*;

type Mask = Vec<bool>;

type Set = Vec<u8>;
pub struct Lattice {
    n : u8,
    u : Set,
}

#[derive(Debug)]
pub struct Cell {
    n : u8,
    k : u8,
    mask : Mask,
    offset : Set,
    face : Set,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_combinatorics() {
        let l = Lattice::new(3);
        for s in l.k_combinations(3) {
            for c in l.sub_cells(2, &s) {
                println!("{}", c.tokenize())
            }
            println!();
        }

        //l.sub_cells(m, s)
    }
}
