lattice_macros::define_u_n_types!(3);


fn main() {
    let res = combinations(&(1..5).collect(),3);

    println!("{:?} {}", res, binom(4,3));
}
