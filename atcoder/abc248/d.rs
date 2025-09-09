#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
        q: usize,
        ask: [(Usize1, Usize1, usize); q],
    }

    let mut indices = vec![vec![]; n + 1];
    for i in 0..n {
        indices[arr[i]].push(i);
    }

    let mut ans = vec![];
    for &(l, r, x) in &ask {
        let i = indices[x].partition_point(|y| *y < l);
        let j = indices[x].partition_point(|y| *y <= r);
        ans.push(j - i);
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
