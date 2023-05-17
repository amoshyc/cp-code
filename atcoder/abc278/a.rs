use itertools::*;
use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        mut a: [u32; n],
    }
    for _ in 0..k {
        a.remove(0);
        a.push(0);
    }
    println!("{}", a.iter().join(" "));
}
