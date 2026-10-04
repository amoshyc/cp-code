#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        adb: [(usize, Usize1, usize); n],
    }

    let mut events = vec![vec![]; m];
    for &(a, d, b) in &adb {
        events[0].push(('+', a));
        events[d].push(('-', a));
        events[d].push(('+', b));
    }

    let mut cnt = HashMap::new();
    let mut res = vec![0; m];
    for t in 0..m {
        for &(op, c) in events[t].iter() {
            if op == '+' {
                *cnt.entry(c).or_insert(0) += 1;
            } else {
                *cnt.entry(c).or_insert(0) -= 1;
                if cnt[&c] == 0 {
                    cnt.remove(&c);
                }
            }
        }
        res[t] = cnt.len();
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
