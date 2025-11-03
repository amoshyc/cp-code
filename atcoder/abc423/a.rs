#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        x: i64,
        c: i64,
    }

    let ok = |m: i64| -> bool { m * 1000 + m * c <= x };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = x + 1;
    if !ok(lb) {
        println!("0");
        return;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{}", lb * 1000);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
