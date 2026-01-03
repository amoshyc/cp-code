#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }

    // x + k <= z (y + k)
    // x - zy <= (z - 1) k
    // k >= floor_div(x - zy, z - 1)

    let k = (x - z * y).div_euclid(z - 1);
    if k >= 0 && x + k == z * (y + k) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
