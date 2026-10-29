use super::{Set,Mask};

pub fn combinations( s: &Set, k : u8 ) -> Vec<Set> {
    let mut res = Vec::new();
    dfs(s,k,0,&mut Set::new(), &mut res);
    res
}
fn dfs(s: &Set, k:u8, i:usize, subset : &mut Set, res : &mut Vec<Set>) {
    if subset.len() == k as usize {
        res.push( subset.clone() );
        return
    }
    for j in i..s.len() {
        subset.push(s[j]);
        dfs(s,k,j+1, subset, res);
        subset.pop();
    }
}

pub fn binom( n : u8, k : u8 ) -> u64 {
    return factorial(n) / ( factorial(k) * factorial(n-k) )

}

fn factorial(n:u8) -> u64 {
    let mut res = 1;
    for i in 1..=n {
        res *= i as u64;
    }
    res
}

pub fn difference( a : &Set, b : &Set ) -> Set {
    let mut c = Set::new();
    for i in a {
        if !b.contains(i) {
            c.push(*i);
        }
    }
    c
}

pub fn addition( a : &Set, b : &Set ) -> Set {
    let mut c = a.clone();
    c.append(&mut b.clone());
    c
}

pub fn gray_encode( l : u8 ) -> Vec<Mask> {
    let mut res = Vec::new();
    for n in 0..(2u32.pow(l as u32)) {
        let gray = n ^ (n >> 1);
        let b = (0..l).rev().map(|i| (gray >> i) & 1 != 0 );
        res.push(b.collect());
    }
    res
}
