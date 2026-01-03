#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        mut x: i64,
        n: usize,
        w: [i64; n],
        q: usize,
        ask: [Usize1; q],
    }

    let mut ans = vec![];
    let mut f = vec![false; n];
    for p in ask {
        if f[p] {
            f[p] = false;
            x -= w[p];
        } else {
            f[p] = true;
            x += w[p];
        }
        ans.push(x);
    }
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
