#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        k: u64,
    }

    let nn = 1 << n;
    if k % (nn as u64) == 0 {
        let avg = k / (nn as u64);
        let ans = vec![avg; nn];
        println!("0");
        println!("{}", join(&ans, " "));
    } else {
        let mut ans = vec![0; nn];
        dfs(0, nn, k, &mut ans);
        println!("1");
        println!("{}", join(&ans, " "));
    }
}

fn dfs(l: usize, r: usize, k: u64, ans: &mut Vec<u64>) {
    if r - l == 1 {
        ans[l] = k;
        return;
    }
    let m = (l + r) / 2;
    let h = k / 2;
    dfs(l, m, h, ans);
    dfs(m, r, k - h, ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
