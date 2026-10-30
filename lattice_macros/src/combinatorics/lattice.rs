use crate::combinatorics::utils::{gray_encode,binom};

use super::*;

impl Lattice {
    pub fn new( n : u8 ) -> Self {
        Self{ n, u : Set((0..n).collect()) }
    }

    pub fn adj_size( n : u8 , k : u8, m : u8 ) -> usize {
        if k > m {
            ( 2u64.pow((k-m) as u32) * binom(k, m) ) as usize
        } else {
            ( 2u64.pow((m-k) as u32) * binom(n-k, m-k) ) as usize
        }
    }

    pub fn adj_cells( &self, m : u8, s : &Set ) -> Vec<Cell> {
        if s.len() > m as usize {
            self.sub_cells(m, s)
        } else {
            self.sup_cells(m, s)
        }
    }

    /// Generate all incident m-cells for a k-cell's S-face
    /// here: m < k
    /// and |S| = k
    fn sub_cells( &self, m : u8, s : &Set ) -> Vec<Cell> {
        let k = s.len() as u8;
        let mut cells = Vec::new();
        for m_face in self.m_subset(m, s) {
            let offset = difference(s, &m_face);
            let mut cell : Vec<Cell> = gray_encode(offset.len() as u8)
                .iter().map(|mask| {
                Cell::new( self.n, k, m, mask, &offset, &m_face)
            }).collect();
            cells.append(&mut cell);
        }
        cells
    }

    /// Generate all adjacent m-cells for a k-cell's S-face
    /// here: k < m
    /// and |S| = k
    fn sup_cells( &self, m : u8, s : &Set ) -> Vec<Cell> {
        let k = s.len() as u8;
        let mut cells = Vec::new();
        for m_face in self.m_supset(m, s) {
            let offset = difference(&m_face, s);
            let mut cell : Vec<Cell> = gray_encode(offset.len() as u8)
                .iter().map(|mask| {
                Cell::new( self.n, k, m, mask, &offset, &m_face)
            }).collect();
            cells.append(&mut cell);
        }
        cells
    }

    /// Generates all combinations of $U = \{1,...,n\}$
    /// $$
    ///     \{ S \subset U, |s| = k \}
    /// $$
    /// A k-cell is described by (u_0,...,u_n) + some S-face
    pub fn k_combinations( &self, k : u8 ) -> Vec<Set> {
        combinations(&self.u, k)
    }

    /// Generates all combinations of $S \subset U$
    /// $$
    ///     \{ X \subset S, |X| = m \}
    /// $$
    /// For a k-cell X encodes incident m-cells' faces in the S-face
    fn m_subset( &self, m : u8, s : &Set ) -> Vec<Set> {
        combinations(s, m)
    }
    
    /// Generates all supersets of $S \subset U$ in $U$
    /// $$
    ///     \{ Y \superset S, Y \subset U, |Y| = m \}
    /// $$
    /// This is equivalent to finding
    /// $$
    ///     \{ Z + S, Z \subset U-S, |Z| = m - |S| \}
    /// $$
    /// For a k-cell Y encodes adjacent m-cells' faces in the S-face
    fn m_supset( &self, m : u8, s : &Set ) -> Vec<Set> {
        let u_s = difference(&self.u, s);
        combinations(&u_s, m - s.len() as u8).iter().map(|z| addition(s,z)).collect()
    }
}

