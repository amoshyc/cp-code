#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        tc: usize,
        ask: [(i64, i64, i64); tc],
    }

    let mut ans = vec![];
    for &(na, nb, nc) in &ask {
        // It can be solvedy by binary search:
        // let ok = |m: i64| -> bool {
        //     let mut ok = true;
        //     ok &= na >= m;
        //     ok &= nc >= m;
        //     ok &= (na - m) + (nc - m) + nb >= m;
        //     ok
        // };
        // Or greedy:
        // Note that the 3 constraints can be repharesed as:
        //     m <= na,
        //     m <= nc,
        //     3m <= na + nb + nc
        // Therefore:
        let m = na.min(nc).min((na + nb + nc) / 3);
        ans.push(m);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
