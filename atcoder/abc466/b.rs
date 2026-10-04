#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        cs: [(Usize1, isize); n],
    }

    let mut ans = vec![-1; m];
    for &(c, s) in &cs {
        ans[c] = ans[c].max(s);
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
