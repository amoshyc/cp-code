#![allow(unused)]

use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        asks: [(Usize1, Usize1, usize); q],
    }

    // 1  2  3  4  5  6  7  8
    // [] [] [] [] [] [] [] []

    //    1  1  1  1
    // 2  2  2  2
    //                   1  1
    //       3  3  3  3
    //    2  2  2  2

    let mut diff = vec![vec![]; n + 1];
    for (i, &(l, r, x)) in asks.iter().enumerate() {
        diff[l].push(('+', x, i));
        diff[r + 1].push(('-', x, i));
    }

    let mut ans = vec![];
    let mut cnt = HashMap::new();
    for i in 0..n {
        for &(k, x, i) in &diff[i] {
            if k == '+' {
                cnt.entry(x).or_insert(HashSet::new()).insert(i);
            } else {
                cnt.entry(x).or_insert(HashSet::new()).remove(&i);
                if cnt[&x].len() == 0 {
                    cnt.remove(&x);
                }
            }
        }
        ans.push(cnt.len());
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
